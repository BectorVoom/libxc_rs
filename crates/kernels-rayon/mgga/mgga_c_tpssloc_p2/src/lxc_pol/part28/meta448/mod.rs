//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta448 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1637;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1638;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1639;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta448(t23967: f64, t6492: f64, t2031: f64, t22550: f64, t6495: f64, t7032: f64, t7025: f64, t9231: f64, t6486: f64, t240: f64, t67: f64, t1864: f64, t1860: f64, t6509: f64, t7031: f64, t22489: f64, t2032: f64, t22493: f64, t22519: f64, t22527: f64, t22531: f64, t22534: f64, t22537: f64, t22546: f64, t22549: f64, t23963: f64, t7026: f64, t7035: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23968, t23970, t23973, t23975, t23978, t23992, t23993) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1637(t23967, t6492, t2031, t22550, t6495, t7032, t7025, t9231, t6486, t240, t67, t1864);
        let (t23995, t23998) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1638(t1860, t23993, t6509, t7031);
        let (t23999, t24001, t24006) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1639(t1860, t23998, t2031, t22489, t2032, t22493, t22519, t22527, t22531, t22534, t22537, t22546, t22549, t23963, t23968, t23970, t23973, t23975, t23978, t23995, t6486, t6492, t6495, t7026, t7035);
    (t23968, t23970, t23973, t23975, t23978, t23992, t23993, t23995, t23998, t23999, t24001, t24006)
}
