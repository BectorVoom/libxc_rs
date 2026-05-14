//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1287/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1287<F: Float>(t10916: F, t10917: F, t11199: F, t11200: F, t11211: F, t2303: F, t2318: F, t2326: F, t2333: F, t31393: F, t3447: F, t3453: F, t3462: F, t4242: F, t6960: F, t6965: F, t6981: F, t855: F, t8902: F, t8934: F, t8941: F, t9230: F, t9234: F) -> (F,) {
    let t31556 = 0.11696447245269292414e1 * t855 * t11211 * t2318 + 0.20779030926817756511e3 * t2333 * t10917 + 0.2077903092681775651e3 * t3447 * t8941 - 0.69263436422725855034e2 * t3447 * t8902 + 0.14035736694323150897e2 * t855 * t10916 * t2303 + 0.10389515463408878255e3 * t855 * t6981 * t4242 * t2326 - 0.10254018858216406658e4 * t855 * t6965 * t4242 * t9230 + 0.46785788981077169656e1 * t8934 * t3453 - 0.17315859105681463759e2 * t855 * t11199 * t6960 - 0.69263436422725855034e2 * t8934 * t3462 - 0.34631718211362927518e2 * t2333 * t11200 + t31393 - 0.34631718211362927517e2 * t3447 * t9234;
    (t31556,)
}
