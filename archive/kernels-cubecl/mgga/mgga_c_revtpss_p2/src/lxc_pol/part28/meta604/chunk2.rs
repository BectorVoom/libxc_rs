//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2086/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2086<F: Float>(t25894: F, t97703: F, t97705: F, t25921: F, t25924: F, t25966: F, t26046: F, t27837: F, t27841: F, t4131: F, t7295: F, t7920: F, t94378: F, t94388: F, t94392: F, t94399: F, t97682: F, t97687: F, t97690: F, t97698: F, t97702: F) -> F {
    let t97707 = F::cast_from(0.14456046980341999104e-1_f64) * t25894 * t97703 * t97705;
    let t97716 = -t97682 + t97687 + t97690 - F::cast_from(0.26020884564615598386e1_f64) * t7295 * t25924 * t7920 * t4131 + F::cast_from(0.4336814094102599731e0_f64) * t27837 * t25966 - t97698 - t97702 - t97707 + F::cast_from(0.4336814094102599731e0_f64) * t27837 * t26046 - F::cast_from(0.19274729307122665471e-1_f64) * t94378 - F::cast_from(0.52041769129231196772e1_f64) * t25921 * t27841 - F::cast_from(0.34270468708064099208e-2_f64) * t94388 + F::cast_from(0.45699670022203476294e-2_f64) * t94392 + F::cast_from(0.28912093960683998208e-1_f64) * t94399;
    t97716
}
