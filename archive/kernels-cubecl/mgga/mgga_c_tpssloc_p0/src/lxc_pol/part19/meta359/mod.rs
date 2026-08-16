//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta359 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1304;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1305;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1306;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1307;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta359<F: Float>(t41666: F, t42308: F, t10321: F, t1041: F, t248: F, t3051: F, t10459: F, t3117: F, t10469: F, t990: F, t10471: F, t10875: F, t10214: F, t10378: F, t10463: F, t10863: F, t10879: F, t2960: F, t2979: F, t3062: F, t3098: F, t39097: F, t41644: F, t41693: F, t41697: F, t41701: F, t41705: F, t42303: F, t973: F, t974: F, t977: F, t10468: F, t191: F, t349: F, t68: F) -> (F, F, F, F, F, F) {
        let (t42309, t42322, t42324, t42332, t42333, t42334) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1304::<F>(t41666, t42308, t10321, t1041, t248, t3051, t10459, t3117, t10469, t990, t10471, t10875);
        let t42337 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1305::<F>(t10214, t10378, t1041, t10463, t10863, t10879, t248, t2960, t2979, t3062, t3098, t3117, t39097, t41644, t41693, t41697, t41701, t41705, t42303, t42309, t42322, t42324, t42334, t973, t974, t977);
        let (t42339, t42340) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1306::<F>(t10468, t191, t349);
        let t42341 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1307::<F>(t10471, t68);
    (t42332, t42333, t42337, t42339, t42340, t42341)
}
