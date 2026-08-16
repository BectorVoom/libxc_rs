//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta705 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2204;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2205;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta705(t26161: f64, t26163: f64, t97920: f64, t24991: f64, t7685: f64, t22574: f64, t25988: f64, t33136: f64, t28823: f64, t6876: f64, t1874: f64, t96709: f64, t19534: f64, t89: f64, t28030: f64, t6525: f64, t28821: f64, t6880: f64, t28239: f64, t1983: f64, t26503: f64, t5161: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t97923, t97925, t97928, t97930, t97932) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2204(t26161, t26163, t97920, t24991, t7685, t22574, t25988, t33136, t28823, t6876, t1874, t96709);
        let (t97935, t97937, t97941, t97942, t97947) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2205(t19534, t89, t1874, t28030, t6525, t28821, t6880, t28239, t6876, t1983, t26503, t5161);
    (t97923, t97925, t97928, t97930, t97932, t97935, t97937, t97941, t97942, t97947)
}
