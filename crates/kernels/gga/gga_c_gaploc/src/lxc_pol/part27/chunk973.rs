//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 973/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk973<F: Float>(t7173: F, t900: F, t2683: F, t6099: F, t1964: F, t9419: F, t1984: F, t823: F, t15478: F, t5638: F, t822: F, t2089: F, t40: F, t7291: F, t15479: F, t10007: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t22405 = t900 * t7173;
    let t22424 = t6099 * t2683;
    let t22537 = t1964 * t9419;
    let t22538 = t1984 * t22537;
    let t22542 = t823 * t9419;
    let t22543 = t1984 * t22542;
    let t22622 = t822 * t5638 * t15478;
    let t22623 = t40 * t2089;
    let t22624 = t22623 * t7291;
    let t22628 = t822 * t15479;
    let t22629 = t10007 * t7291;
    (t22405, t22424, t22537, t22538, t22542, t22543, t22622, t22623, t22624, t22628, t22629)
}
