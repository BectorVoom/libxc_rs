//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta358 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1155;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1156;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta358(t42112: f64, t2859: f64, t2884: f64, t302: f64, t41654: f64, t41961: f64, t2887: f64, t271: f64, t2770: f64, t41666: f64, t10468: f64, t191: f64, t349: f64, t10471: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42113, t42154, t42212, t42213, t42226, t42228, t42245, t42308, t42309, t42339) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1155(t42112, t2859, t2884, t302, t41654, t41961, t2887, t271, t2770, t41666, t10468, t191);
        let (t42340, t42341) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1156(t349, t42339, t10471, t68);
    (t42113, t42154, t42212, t42213, t42226, t42228, t42245, t42308, t42309, t42339, t42340, t42341)
}
