//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1061/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1061<F: Float>(t2268: F, t2343: F, t31688: F, t31690: F, t31692: F, t31695: F, t31698: F, t31701: F, t31704: F, t31706: F, t31708: F, t31710: F, t31714: F, t31715: F, t31719: F, t31724: F, t31727: F, t6320: F) -> (F,) {
    let t31728 = -t31688 - t31690 - t31692 - t31695 + t31698 - t31701 - t31704 + t31706 + t31708 + t31710 + t31714 + 0.34146007962811379518e0 * t2268 * t2343 * t31715 - 0.17073003981405689759e0 * t2268 * t6320 * t31719 + t31724 + t31727;
    (t31728,)
}
