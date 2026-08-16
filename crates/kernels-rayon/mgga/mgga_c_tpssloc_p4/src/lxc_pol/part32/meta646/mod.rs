//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta646 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2067;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2068;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta646(t26447: f64, t90607: f64, t90787: f64, t22751: f64, t26397: f64, t22892: f64, t22893: f64, t26396: f64, t26384: f64, t26388: f64, t7733: f64, t81186: f64, t5318: f64, t552: f64, t5187: f64, t562: f64, t26392: f64, t80670: f64, t22705: f64, t26422: f64, t81228: f64, t22704: f64, t26466: f64, t26461: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90789, t90792, t90795, t90798, t90806, t90807) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2067(t26447, t90607, t90787, t22751, t26397, t22892, t22893, t26396, t26384, t26388, t7733, t81186);
        let (t90809, t90818, t90837, t90845, t90860, t90864) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2068(t5318, t552, t5187, t562, t26392, t80670, t22705, t26422, t81228, t22704, t26466, t26461);
    (t90789, t90792, t90795, t90798, t90806, t90807, t90809, t90818, t90837, t90845, t90860, t90864)
}
