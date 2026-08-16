//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 914/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk914(t1024: f64, t1634: f64, t581: f64, t1702: f64, t2587: f64, t50: f64, t6853: f64, t1025: f64, t5264: f64, t1769: f64, t2667: f64, t1706: f64, t2592: f64, t5225: f64, t5244: f64, t5265: f64, t5267: f64, t5289: f64, t580: f64, t6904: f64, t6908: f64, t6914: f64, t6916: f64, t6920: f64) -> (f64, f64, f64, f64) {
    let t6924 = t581 * t1024 * t1634;
    let t6928 = 7.0_f64 / 72.0_f64 * t1702 * t2587;
    let t6929 = t50 * t6853;
    let t6930 = t581 * t6929;
    let t6933 = t5264 * t1025;
    let t6935 = t1769 * t2667;
    let t6937 = 0.12862205435420921092e-2_f64 * t2592 * t6904 - 0.17149607247227894789e-2_f64 * t5244 * t6908 - 35.0_f64 / 108.0_f64 * t5265 + 7.0_f64 / 144.0_f64 * t5267 - t6914 + t1706 * t6916 / 8.0_f64 + t1706 * t6920 / 16.0_f64 - t5225 * t6924 / 4.0_f64 + t6928 - t580 * t6930 / 48.0_f64 - 35.0_f64 / 216.0_f64 * t6933 + 0.80031500487063509014e-2_f64 * t6935 - t5289;
    (t6924, t6929, t6930, t6937)
}
