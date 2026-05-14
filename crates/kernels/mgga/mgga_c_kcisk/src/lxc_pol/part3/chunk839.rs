//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 839/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk839<F: Float>(t13829: F, t398: F, t1322: F, t3961: F, t1310: F, t1293: F, t3969: F, t1300: F, t4154: F, t1309: F, t1324: F, t13805: F, t13807: F, t13810: F, t13817: F, t13821: F, t13824: F, t13827: F, t3963: F, t3966: F, t3970: F, t3990: F, t405: F) -> (F, F) {
    let t13830 = 1.0 / t13829;
    let t13831 = t398 * t13830;
    let t13832 = t3961 * t1322;
    let t13833 = t13831 * t13832;
    let t13834 = t1310 * t13833;
    let t13839 = t1293 * t3969;
    let t13846 = t4154 * t1300;
    let t13849 = 0.35981577432354634426e-1 * t13805 - 0.10794473229706390328e0 * t13807 - 0.53972366148531951639e-1 * t13810 - 0.16191709844559585492e0 * t3966 * t3990 - 0.5397236614853195164e-1 * t1309 * t13817 - 0.15831894070236039148e1 * t13821 * t1324 + 0.28785261945883707541e0 * t13824 + 0.10794473229706390328e0 * t13827 - 0.32383419689119170984e0 * t1309 * t13834 - 0.86355785837651122625e0 * t3970 * t3963 + 0.86355785837651122625e0 * t13839 * t1324 + 0.43177892918825561313e0 * t3970 * t3990 + 0.32383419689119170984e0 * t3966 * t3963 - 0.43177892918825561313e0 * t13846 * t405;
    (t13839, t13849)
}
