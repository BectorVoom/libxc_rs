//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta352 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1661;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1662;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1663;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta352(t12240: f64, t6977: f64, t3851: f64, t3901: f64, t1337: f64, t562: f64, t3792: f64, t550: f64, t12177: f64, t3897: f64, t1338: f64, t3879: f64, t1352: f64, t3773: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12241, t12244, t12247, t12248, t12249, t12250, t12251) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1661(t12240, t6977, t3851, t3901, t1337, t562, t3792, t550, t12177);
        let (t12252, t12255) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1662(t12249, t12251, t12177, t3792);
        let (t12256, t12259, t12260, t12267) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1663(t12255, t3897, t1338, t3879, t1352, t3773, t68);
    (t12241, t12244, t12247, t12248, t12250, t12251, t12252, t12255, t12256, t12259, t12260, t12267)
}
