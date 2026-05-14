//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1201/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1201<F: Float>(t30003: F, t30005: F, t31688: F, t31690: F, t31692: F, t31695: F, t31698: F, t31701: F, t31704: F, t31706: F, t31708: F, t31710: F, t31714: F, t31724: F, t31727: F, t1570: F, t3689: F) -> (F, F) {
    let t38355 = -t31688 - t31690 - t31692 - t31695 + t31698 - t31701 - t31704 + t31706 + t31708 + t31710 + t31714 + t31724 + t31727 + t30003 - t30005;
    let t38362 = t1570 * t3689;
    (t38355, t38362)
}
