//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 638/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk638<F: Float>(t11: F, t1643: F, t1645: F, t2736: F, t2804: F, t2819: F, t2828: F, t5: F) -> (F,) {
    let t2832 = t1643 - 5.0 / 3.0 * t1645 - 5.0 / 3.0 * t2736 + 5.0 * t5 * t11 * t2804 - 45.0 * param_eta * (t2819 + t2828);
    (t2832,)
}
