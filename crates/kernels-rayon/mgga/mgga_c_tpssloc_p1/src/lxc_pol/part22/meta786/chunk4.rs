//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2719/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2719(t1851: f64, t671: f64, t12524: f64, t1395: f64, t1401: f64, t1458: f64, t16521: f64, t16524: f64, t19534: f64, t20162: f64, t20173: f64, t20176: f64, t20181: f64, t20347: f64, t22445: f64, t22448: f64, t28893: f64, t3938: f64, t3941: f64, t4072: f64, t5371: f64, t5376: f64, t5456: f64, t5493: f64, t55353: f64, t55388: f64, t577: f64, t66958: f64, t75701: f64, t75764: f64, t75784: f64) -> f64 {
    let t75795 = t1851 * t671;
    let t75827 = 0.45e1_f64 * t75764 * t577 + 0.135e2_f64 * t75784 * t671 + 0.405e2_f64 * t66958 * t1458 + 81.0_f64 * t55388 * t5376 + 0.405e2_f64 * t20162 * t4072 + 81.0_f64 * t55353 * t5456 + 81.0_f64 * t75795 * t5456 + 162.0_f64 * t16524 * t20176 + 0.405e2_f64 * t16521 * t5493 + 81.0_f64 * t16524 * t20181 + 0.405e2_f64 * t5371 * t19534 + 27.0_f64 * t1395 * t22445 + 81.0_f64 * t28893 * t4072 + 81.0_f64 * t12524 * t22448 + 81.0_f64 * t20173 * t22448 + 81.0_f64 * t3941 * t4072 * t5493 + 81.0_f64 * t3941 * t1458 * t19534 + 0.135e2_f64 * t3938 * t20347 + 27.0_f64 * t3941 * t20347 * t671 + 0.135e2_f64 * t1401 * t75701;
    t75827
}
