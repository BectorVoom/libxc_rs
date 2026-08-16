//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta253 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1371;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1372;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1373;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta253(t154: f64, t3584: f64, t3241: f64, t636: f64, t52: f64, t1094: f64, t3312: f64, t3311: f64, t419: f64, t409: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t11145, t11147) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1371(t154, t3584, t3241, t636);
        let (t11152, t11153) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1372(t3241, t52);
        let (t11185, t11189, t11190) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1373(t1094, t3312, t3311, t419, t409);
    (t11145, t11147, t11152, t11153, t11185, t11189, t11190)
}
