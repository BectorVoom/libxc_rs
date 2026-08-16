//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta94 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk647;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk648;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk649;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta94(t2371: f64, t761: f64, t118: f64, t187: f64, t677: f64, t763: f64, t200: f64, t262: f64, t776: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t2373, t2374) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk647(t2371, t761, t118, t187);
        let t2375 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk648(t677, t763);
        let (t2377, t2378, t2379) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk649(t2374, t2375, t200, t262, t776);
    (t2373, t2374, t2375, t2377, t2378, t2379)
}
