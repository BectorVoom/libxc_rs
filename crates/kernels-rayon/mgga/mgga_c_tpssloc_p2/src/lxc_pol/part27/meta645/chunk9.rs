//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2216/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2216(t10469: f64, t23470: f64, t3: f64, t82986: f64, t23437: f64, t4630: f64, t25641: f64, t82943: f64, t1933: f64, t1937: f64, t3966: f64, t14222: f64, t1597: f64, t1622: f64, t23544: f64, t23548: f64, t25580: f64, t25600: f64, t25601: f64, t25658: f64, t3032: f64, t3040: f64, t3098: f64, t360: f64, t4579: f64, t4636: f64, t6722: f64, t6729: f64, t6735: f64, t83071: f64, t83075: f64, t83215: f64, t83220: f64) -> (f64, f64) {
    let t88537 = t82986 * t3 * t23470 * t10469;
    let t88548 = t23437 * t4630 / 216.0_f64;
    let t88566 = 0.16149102437656156342e-2_f64 * t82943 * t25641;
    let t88569 = 0.20186378047070195428e-3_f64 * t1933 * t3966 * t1937;
    let t88570 = 0.10093189023535097714e-3_f64 * t88537 * t25658 * t3032 * t3040 * t360 + t83071 * t1622 / 2304.0_f64 + t23544 * t4636 / 1152.0_f64 - t88548 - t25580 * t3098 / 1152.0_f64 - 0.16149102437656156342e-2_f64 * t83075 - t83220 * t4579 / 216.0_f64 + 0.16149102437656156342e-2_f64 * t6722 * t25600 * t6735 - 0.20186378047070195428e-3_f64 * t1933 * t6729 * t1597 * t6735 - t83215 * t14222 / 1152.0_f64 - 0.10093189023535097714e-3_f64 * t25601 * t23548 - t88566 + t88569;
    (t88537, t88570)
}
