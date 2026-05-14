//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1222/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1222<F: Float>(t26182: F, t146: F, t20946: F, t252: F, t6398: F, t7614: F, t7615: F, t24074: F, t8243: F, t2592: F, t5147: F, t5148: F, t20622: F, t928: F, t20137: F, t6209: F, t8128: F) -> (F, F, F, F, F, F, F) {
    let t26183 = 0.6112917064160653851e0 * t26182;
    let t26185 = t146 * t20946 * t252;
    let t26191 = t7614 * t6398 * t7615;
    let t26193 = t24074 * t8243;
    let t26234 = t5147 * t5148 * t2592;
    let t26238 = t20622 * t928;
    let t26249 = t6209 * t20137 * t8128;
    (t26183, t26185, t26191, t26193, t26234, t26238, t26249)
}
