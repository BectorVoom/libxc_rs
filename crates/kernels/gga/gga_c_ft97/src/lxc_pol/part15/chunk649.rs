//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 649/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk649<F: Float>(t452: F, t4623: F, t942: F, t3119: F, t4533: F, t91: F, t11043: F, t15606: F, t15609: F, t15612: F, t15891: F, t15894: F, t15899: F, t20101: F, t20116: F, t20136: F, t20151: F, t20159: F) -> (F, F, F) {
    let t20307 = t452 * t4623 * t942;
    let t20316 = t91 * t3119 * t4533;
    let t20322 = 2.0 / 9.0 * t15606 - 2.0 / 3.0 * t15609 + t15612 / 3.0 - t20101 - 6.0 * t20116 + t15891 - 2.0 * t15894 - 2.0 / 3.0 * t15899 - 3.0 / 4.0 * t20316 + 4.0 / 3.0 * t20136 - 2.0 * t20151 - t20159 / 3.0 - 4.0 / 9.0 * t11043;
    (t20307, t20316, t20322)
}
