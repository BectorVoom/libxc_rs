//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1062/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1062<F: Float>(t2089: F, t40: F, t7291: F, t15479: F, t822: F, t10007: F, t15488: F, t10012: F, t1410: F, t835: F, t579: F, t2683: F, t5654: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t22623 = t40 * t2089;
    let t22624 = t22623 * t7291;
    let t22628 = t822 * t15479;
    let t22629 = t10007 * t7291;
    let t22633 = t822 * t15488;
    let t22634 = t10012 * t7291;
    let t22672 = t1410 * t835;
    let t22693 = t579 * t2089;
    let t22706 = t579 * t835;
    let t22748 = t5654 * t2683;
    (t22623, t22624, t22628, t22629, t22633, t22634, t22672, t22693, t22706, t22748)
}
