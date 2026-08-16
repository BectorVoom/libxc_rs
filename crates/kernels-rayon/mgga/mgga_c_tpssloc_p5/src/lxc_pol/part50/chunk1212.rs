//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1212/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1212(t23384: f64, t32987: f64, t30781: f64, t7560: f64, t225: f64, t33007: f64, t33005: f64, t1052: f64, t1065: f64, t1066: f64, t113296: f64, t113468: f64, t113600: f64, t119351: f64, t1599: f64, t1634: f64, t1956: f64, t23346: f64, t23365: f64, t23369: f64, t25453: f64, t25778: f64, t30778: f64, t30861: f64, t30899: f64, t3174: f64, t32964: f64, t32992: f64, t349: f64, t388: f64, t4542: f64, t4660: f64, t6687: f64, t6771: f64, t6816: f64, t7600: f64, t89620: f64, t986: f64) -> f64 {
    let t119495 = t23384 * t32987;
    let t119503 = t7560 * t30781;
    let t119523 = t33007 * t225;
    let t119527 = t33005 * t225;
    let t119529 = t349 * t119351 * t388 + 2.0_f64 * t1052 * t3174 * t30899 * t1634 - 0.14621636149762012769e-1_f64 * t23346 * t32987 + 0.18277045187202515961e-2_f64 * t119495 + 2.0_f64 * t4660 * t30778 + 2.0_f64 * t1052 * t3174 * t32964 * t1065 + 0.16449340668482264365e-1_f64 * t6687 * t986 * t119503 - 2.0_f64 * t25778 * t6816 + 4.0_f64 * t6771 * t25453 - 0.16449340668482264365e-1_f64 * t6687 * t23365 * t32992 - 0.54831135561607547883e-2_f64 * t113468 - 2.0_f64 * t89620 * t1956 + 0.16449340668482264365e-1_f64 * t6687 * t4542 * t30861 + 0.16449340668482264365e-1_f64 * t6687 * t1599 * t113296 - t119523 * t1066 + 4.0_f64 * t23369 * t7600 - t119527 * t1066 + t113600;
    t119529
}
