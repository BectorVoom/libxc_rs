//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2198/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2198<F: Float>(t108307: F, t7284: F, t30100: F, t689: F, t25904: F, t25899: F, t25924: F, t27837: F, t27853: F, t27858: F, t27864: F, t5774: F, t7295: F, t7920: F, t94700: F, t94703: F, t94714: F, t94726: F, t94733: F, t94823: F, t97943: F, t97945: F, t97949: F, t98340: F) -> F {
    let t108332 = t7284 * t108307;
    let t108334 = t30100 * t689;
    let t108335 = t25904 * t108334;
    let t108337 = t25899 * t108334;
    let t108349 = t94700 - t94703 - F::cast_from(0.73171657588172351096e-2_f64) * t94714 + F::cast_from(0.52041769129231196772e1_f64) * t94823 * t98340 * t27864 + F::cast_from(0.72280234901709995518e-2_f64) * t108332 - F::cast_from(0.14456046980341999104e-1_f64) * t108335 + F::cast_from(0.25702851531048074406e-1_f64) * t108337 - F::cast_from(0.52041769129231196772e1_f64) * t7295 * t25924 * t7920 * t5774 - F::cast_from(0.11565819519348392139e-2_f64) * t94726 + F::cast_from(0.8673628188205199462e0_f64) * t27837 * t27853 + F::cast_from(0.8673628188205199462e0_f64) * t27837 * t27858 + t97943 + t97945 - F::cast_from(0.65049603595885220126e-3_f64) * t94733 - t97949;
    t108349
}
