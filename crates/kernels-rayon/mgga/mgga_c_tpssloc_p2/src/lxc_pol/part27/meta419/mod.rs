//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta419 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1725;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1726;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta419(t6505: f64, t6509: f64, t2235: f64, t608: f64, t33: f64, t6504: f64, t2240: f64, t641: f64, t645: f64, t72: f64, t2307: f64, t79: f64, t2244: f64, t605: f64, t2251: f64, t6489: f64, t9239: f64, t2241: f64, t1864: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22516, t22519, t22522, t22523, t22527, t22530) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1725(t6505, t6509, t2235, t608, t33, t6504, t2240, t641, t645, t72, t2307, t79);
        let (t22531, t22534, t22537, t22544, t22546, t22549, t22550) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1726(t22530, t72, t2244, t605, t2251, t6489, t9239, t2241, t79, t2240, t608, t1864, t645);
    (t22516, t22519, t22522, t22523, t22527, t22531, t22534, t22537, t22544, t22546, t22549, t22550)
}
