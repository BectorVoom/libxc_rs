//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta313 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1596;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1597;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1598;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta313<F: Float>(t11249: F, t13045: F, t13040: F, t3597: F, t13036: F, t3603: F, t13032: F, t3609: F, t1244: F, t471: F, t3367: F, t414: F, t66: F, t11239: F, t1243: F, t460: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t13046, t13051, t13052, t13053, t13058, t13061, t13062, t13063, t13099) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1596::<F>(t11249, t13045, t13040, t3597, t13036, t3603, t13032, t3609, t1244, t471, t3367, t414);
        let (t13100, t13126) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1597::<F>(t13099, t66, t11239, t1243);
        let t13127 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1598::<F>(t13126, t460);
    (t13046, t13051, t13052, t13053, t13058, t13061, t13062, t13063, t13099, t13100, t13126, t13127)
}
