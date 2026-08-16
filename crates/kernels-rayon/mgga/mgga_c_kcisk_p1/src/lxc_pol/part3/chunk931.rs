//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 931/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk931(t13790: f64, t395: f64, t1308: f64, t4154: f64, t1322: f64, t3959: f64, t3988: f64, t6204: f64, t1309: f64, t1315: f64, t13474: f64, t13478: f64, t13482: f64, t13487: f64, t13489: f64, t13493: f64, t13497: f64, t13501: f64, t13506: f64, t13509: f64, t3935: f64, t3939: f64, t405: f64, sigma0: f64) -> (f64, f64) {
    let t13791 = t13790 * sigma0;
    let t13792 = t13791 * t395;
    let t13795 = t4154 * t1308;
    let t13799 = t3959 * t1322 * t3988;
    let t13800 = t6204 * t13799;
    let t13803 = 0.10794473229706390328e0_f64 * t3935 * t13474 + 0.10794473229706390328e0_f64 * t3935 * t13478 + 0.28785261945883707542e0_f64 * t13482 * t3939 - 0.35981577432354634425e-1_f64 * t13487 + 0.71963154864709268853e-1_f64 * t3935 * t13489 - 0.10794473229706390328e0_f64 * t13493 * t3939 - 0.53972366148531951639e-1_f64 * t3935 * t13497 - 0.53972366148531951639e-1_f64 * t3935 * t13501 - 0.71963154864709268852e-1_f64 * t3935 * t13506 - 0.10794473229706390328e0_f64 * t3935 * t13509 + 0.5397236614853195164e-1_f64 * t13792 * t405 + 0.53972366148531951639e-1_f64 * t13795 * t1315 + 0.32383419689119170984e0_f64 * t1309 * t13800;
    (t13795, t13803)
}
