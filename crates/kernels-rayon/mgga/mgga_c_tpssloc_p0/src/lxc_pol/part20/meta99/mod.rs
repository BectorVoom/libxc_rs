//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta99 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk670;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk671;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk672;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta99(t2471: f64, t731: f64, t723: f64, t159: f64, t167: f64, t2461: f64, t676: f64, t682: f64, t268: f64, t703: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2472, t2475) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk670(t2471, t731, t723);
        let (t2476, t2477, t2478, t2479) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk671(t2475, t159, t167);
        let (t2480, t2483, t2486) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk672(t2461, t2479, t676, t682, t268, t703);
    (t2472, t2475, t2476, t2477, t2478, t2479, t2480, t2483, t2486)
}
