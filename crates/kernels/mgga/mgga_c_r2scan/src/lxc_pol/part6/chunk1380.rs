//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1380/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1380<F: Float>(t2148: F, t26185: F, t26186: F, t6398: F, t7614: F, t7615: F, t24074: F, t8243: F, t2155: F, t25670: F, t6063: F, t25963: F, t6155: F, t24714: F, t6086: F, t6535: F) -> (F, F, F, F, F, F) {
    let t26188 = t26185 * t2148 * t26186;
    let t26191 = t7614 * t6398 * t7615;
    let t26193 = t24074 * t8243;
    let t26196 = t2155 * t6063 * t25670;
    let t26198 = t6155 * t25963;
    let t26201 = t6535 * t6086 * t24714;
    (t26188, t26191, t26193, t26196, t26198, t26201)
}
