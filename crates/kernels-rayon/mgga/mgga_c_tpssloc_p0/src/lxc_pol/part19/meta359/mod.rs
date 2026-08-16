//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta359 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1304;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1305;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1306;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1307;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta359(t41666: f64, t42308: f64, t10321: f64, t1041: f64, t248: f64, t3051: f64, t10459: f64, t3117: f64, t10469: f64, t990: f64, t10471: f64, t10875: f64, t10214: f64, t10378: f64, t10463: f64, t10863: f64, t10879: f64, t2960: f64, t2979: f64, t3062: f64, t3098: f64, t39097: f64, t41644: f64, t41693: f64, t41697: f64, t41701: f64, t41705: f64, t42303: f64, t973: f64, t974: f64, t977: f64, t10468: f64, t191: f64, t349: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t42309, t42322, t42324, t42332, t42333, t42334) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1304(t41666, t42308, t10321, t1041, t248, t3051, t10459, t3117, t10469, t990, t10471, t10875);
        let t42337 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1305(t10214, t10378, t1041, t10463, t10863, t10879, t248, t2960, t2979, t3062, t3098, t3117, t39097, t41644, t41693, t41697, t41701, t41705, t42303, t42309, t42322, t42324, t42334, t973, t974, t977);
        let (t42339, t42340) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1306(t10468, t191, t349);
        let t42341 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1307(t10471, t68);
    (t42332, t42333, t42337, t42339, t42340, t42341)
}
