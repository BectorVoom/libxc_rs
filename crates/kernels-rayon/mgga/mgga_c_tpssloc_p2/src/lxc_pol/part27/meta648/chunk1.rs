//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2239/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2239(t4541: f64, t984: f64, t23384: f64, t25467: f64, t25459: f64, t1058: f64, t1060: f64, t11037: f64, t13933: f64, t14526: f64, t1615: f64, t1920: f64, t1948: f64, t1949: f64, t23346: f64, t23571: f64, t23670: f64, t25541: f64, t25558: f64, t25713: f64, t25718: f64, t3076: f64, t3186: f64, t3188: f64, t345: f64, t6687: f64, t7622: f64, t88941: f64, t89312: f64) -> (f64, f64) {
    let t89349 = t4541 * t984;
    let t89360 = 0.54831135561607547884e-2_f64 * t23384 * t25467;
    let t89362 = 0.54831135561607547884e-2_f64 * t23384 * t25459;
    let t89363 = 0.82246703342411321825e-2_f64 * t1920 * t345 * t1948 * t14526 - 0.14621636149762012769e-1_f64 * t23346 * t25718 + t3076 * t7622 - 2.0_f64 * t11037 * t25558 + 2.0_f64 * t3186 * t89312 * t3188 + t1058 * t23571 * t1615 * t1060 - 0.43864908449286038306e-1_f64 * t23670 * t25541 - 0.16449340668482264365e-1_f64 * t6687 * t89349 * t25713 - 0.82246703342411321825e-2_f64 * t6687 * t88941 * t25713 - 0.82246703342411321825e-2_f64 * t6687 * t13933 * t1949 - t89360 - t89362;
    (t89349, t89363)
}
