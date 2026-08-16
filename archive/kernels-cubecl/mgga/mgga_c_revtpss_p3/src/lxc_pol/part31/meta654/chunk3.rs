//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2189/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2189<F: Float>(t30105: F, t689: F, t94395: F, t94649: F, t108133: F, t108135: F, t30071: F, t7308: F, t94378: F, t94388: F, t94392: F, t97682: F, t97687: F, t97690: F, t97698: F, t97702: F, t97707: F) -> F {
    let t108138 = t30105 * t689;
    let t108139 = t94395 * t108138;
    let t108141 = t94649 * t108138;
    let t108145 = -t97682 + t97687 + t97690 - F::cast_from(0.4336814094102599731e0_f64) * t30071 * t7308 - t97698 - F::cast_from(0.72280234901709995518e-2_f64) * t108133 + F::cast_from(0.12851425765524037203e-1_f64) * t108135 - t97702 - t97707 - F::cast_from(0.96373646535613327357e-2_f64) * t94378 + F::cast_from(0.28912093960683998207e-1_f64) * t108139 - F::cast_from(0.51405703062096148813e-1_f64) * t108141 - F::cast_from(0.17135234354032049604e-2_f64) * t94388 + F::cast_from(0.22849835011101738147e-2_f64) * t94392;
    t108145
}
