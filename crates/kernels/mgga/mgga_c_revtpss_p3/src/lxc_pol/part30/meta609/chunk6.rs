//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2081/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2081<F: Float>(t25894: F, t97676: F, t97680: F, t1444: F, t5659: F, t14110: F, t94901: F, t10073: F, t1903: F, t2029: F, t25929: F, t25930: F, t25931: F, t27868: F, t49306: F, t94635: F, t94641: F, t94648: F, t94650: F, t94662: F, t94665: F, t94672: F, t94675: F, t94677: F) -> F {
    let t97838 = F::cast_from(0.28912093960683998208e-1_f64) * t25894 * t97676 * t97680;
    let t97839 = t5659 * t1444;
    let t97843 = t94901 * t14110;
    let t97847 = t10073 * t25929 * t2029 * t1903;
    let t97854 = F::cast_from(0.4336814094102599731e0_f64) * t27868 * t25931 * t49306 - F::cast_from(0.34270468708064099208e-1_f64) * t94635 + F::cast_from(0.12851425765524037203e-1_f64) * t94641 + t94648 - F::cast_from(0.51405703062096148812e-1_f64) * t94650 + t97838 - F::cast_from(0.17347256376410398924e1_f64) * t25930 * t25931 * t97839 + F::cast_from(0.39029762157531132075e-1_f64) * t97843 + F::cast_from(0.4818682326780666368e-3_f64) * t97847 + F::cast_from(0.38549458614245330943e-1_f64) * t94662 - F::cast_from(0.14456046980341999104e-1_f64) * t94665 - F::cast_from(0.77108554593144223218e-1_f64) * t94672 + F::cast_from(0.43368140941025997312e-1_f64) * t94675 + F::cast_from(0.34270468708064099208e-1_f64) * t94677;
    t97854
}
