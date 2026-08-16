//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1053/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1053(t264: f64, t259: f64, t15613: f64, t15626: f64, t1121: f64, t397: f64, t1118: f64, t3392: f64, t1111: f64, t3405: f64, t12630: f64, t67: f64, t10: f64, t1102: f64, t119: f64, t142: f64, t260: f64, t261: f64, t3380: f64, t918: f64) -> (f64, f64, f64, f64, f64) {
    let t265 = t264 < -0.66725e-1_f64;
    let t270 = 0.0_f64 < t259;
    let t15627 = t15613 + t15626;
    let t15629 = piecewise3(t270, t15627, -t15627);
    let t15631 = t397 * t1121 * t15629;
    let t15637 = t3392 * t1118;
    let t15643 = t1111 * t3405;
    let t15646 = t67 * t12630;
    let t15660 = piecewise3(t265, 0.0_f64, 10.0_f64 / 9.0_f64 * t260 * t15646 * t10 - 10.0_f64 / 9.0_f64 * t260 * t3380 * t142 + 40.0_f64 / 27.0_f64 * t260 * t1102 * t119 - 280.0_f64 / 243.0_f64 * t260 * t261 * t918);
    (t15627, t15631, t15637, t15643, t15660)
}
