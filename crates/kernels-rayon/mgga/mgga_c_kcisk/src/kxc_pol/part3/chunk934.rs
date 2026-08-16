//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 934/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk934(t1293: f64, t3969: f64, t1300: f64, t4154: f64, t1309: f64, t1324: f64, t13805: f64, t13807: f64, t13810: f64, t13817: f64, t13821: f64, t13824: f64, t13827: f64, t13834: f64, t3963: f64, t3966: f64, t3970: f64, t3990: f64, t405: f64) -> (f64, f64) {
    let t13839 = t1293 * t3969;
    let t13846 = t4154 * t1300;
    let t13849 = 0.35981577432354634426e-1_f64 * t13805 - 0.10794473229706390328e0_f64 * t13807 - 0.53972366148531951639e-1_f64 * t13810 - 0.16191709844559585492e0_f64 * t3966 * t3990 - 0.5397236614853195164e-1_f64 * t1309 * t13817 - 0.15831894070236039148e1_f64 * t13821 * t1324 + 0.28785261945883707541e0_f64 * t13824 + 0.10794473229706390328e0_f64 * t13827 - 0.32383419689119170984e0_f64 * t1309 * t13834 - 0.86355785837651122625e0_f64 * t3970 * t3963 + 0.86355785837651122625e0_f64 * t13839 * t1324 + 0.43177892918825561313e0_f64 * t3970 * t3990 + 0.32383419689119170984e0_f64 * t3966 * t3963 - 0.43177892918825561313e0_f64 * t13846 * t405;
    (t13839, t13849)
}
