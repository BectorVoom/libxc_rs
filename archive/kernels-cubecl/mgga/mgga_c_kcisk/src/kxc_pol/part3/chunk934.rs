//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 934/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk934<F: Float>(t1293: F, t3969: F, t1300: F, t4154: F, t1309: F, t1324: F, t13805: F, t13807: F, t13810: F, t13817: F, t13821: F, t13824: F, t13827: F, t13834: F, t3963: F, t3966: F, t3970: F, t3990: F, t405: F) -> (F, F) {
    let t13839 = t1293 * t3969;
    let t13846 = t4154 * t1300;
    let t13849 = F::cast_from(0.35981577432354634426e-1_f64) * t13805 - F::cast_from(0.10794473229706390328e0_f64) * t13807 - F::cast_from(0.53972366148531951639e-1_f64) * t13810 - F::cast_from(0.16191709844559585492e0_f64) * t3966 * t3990 - F::cast_from(0.5397236614853195164e-1_f64) * t1309 * t13817 - F::cast_from(0.15831894070236039148e1_f64) * t13821 * t1324 + F::cast_from(0.28785261945883707541e0_f64) * t13824 + F::cast_from(0.10794473229706390328e0_f64) * t13827 - F::cast_from(0.32383419689119170984e0_f64) * t1309 * t13834 - F::cast_from(0.86355785837651122625e0_f64) * t3970 * t3963 + F::cast_from(0.86355785837651122625e0_f64) * t13839 * t1324 + F::cast_from(0.43177892918825561313e0_f64) * t3970 * t3990 + F::cast_from(0.32383419689119170984e0_f64) * t3966 * t3963 - F::cast_from(0.43177892918825561313e0_f64) * t13846 * t405;
    (t13839, t13849)
}
