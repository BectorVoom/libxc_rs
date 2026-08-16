//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1116/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1116(t260: f64, t4305: f64, t10888: f64, t10890: f64, t10893: f64, t10898: f64, t10913: f64, t10915: f64, t10922: f64, t10924: f64, t6969: f64, t7116: f64, t9008: f64, t9189: f64) -> (f64, f64) {
    let t10979 = t260 * t4305;
    let t10992 = 0.19419375e1_f64 * t10888 - 0.258925e1_f64 * t10890 - 0.1294625e1_f64 * t10893 + 0.258925e1_f64 * t10915 - t7116 + 0.40256666666666666667e0_f64 * t6969 + 0.80513333333333333333e0_f64 * t9008 - t9189 - 0.301925e0_f64 * t10898 + 0.905775e0_f64 * t10913 - 0.412621875e-1_f64 * t10922 + 0.16504875e0_f64 * t10924;
    (t10979, t10992)
}
