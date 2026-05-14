//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1338/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1338<F: Float>(t5395: F, t9878: F, t1114: F, t4951: F, t3678: F, t1117: F, t9940: F, t1111: F, t2860: F, t3697: F, t1801: F, t2893: F, t13473: F, t4550: F, t2895: F, t4554: F) -> (F, F, F, F, F, F, F, F, F) {
    let t32688 = t5395 * t9878;
    let t32691 = t4951 * t1114;
    let t32692 = t3678 * t32691;
    let t32695 = t1117 * t9940;
    let t32698 = t4951 * t1111;
    let t32702 = t2860 * t3697;
    let t32710 = t1801 * t2893;
    let t32711 = t32710 * t13473;
    let t32714 = t2893 * t4550;
    let t32715 = t32714 * t2895;
    let t32718 = t2893 * t4554;
    (t32688, t32692, t32695, t32698, t32702, t32710, t32711, t32715, t32718)
}
