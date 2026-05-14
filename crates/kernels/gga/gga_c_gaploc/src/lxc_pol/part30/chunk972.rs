//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 972/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk972<F: Float>(t10007: F, t7291: F, t15488: F, t822: F, t10012: F, t1410: F, t835: F, t2089: F, t579: F, t2683: F, t5654: F, t1890: F, t21783: F, t20157: F, t2085: F, t805: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22629 = t10007 * t7291;
    let t22633 = t822 * t15488;
    let t22634 = t10012 * t7291;
    let t22672 = t1410 * t835;
    let t22693 = t579 * t2089;
    let t22706 = t579 * t835;
    let t22748 = t5654 * t2683;
    let t22775 = t1890 * t21783;
    let t22826 = t805 * t2085 * t20157;
    (t22629, t22633, t22634, t22672, t22693, t22706, t22748, t22775, t22826)
}
