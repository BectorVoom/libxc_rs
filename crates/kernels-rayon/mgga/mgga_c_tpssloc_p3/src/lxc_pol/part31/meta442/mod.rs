//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta442 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1589;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta442(t23967: f64, t6492: f64, t2031: f64, t22550: f64, t6495: f64, t7032: f64, t7025: f64, t9231: f64, t6486: f64, t240: f64, t67: f64, t1864: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t23968, t23970, t23973, t23975, t23978, t23992, t23993) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1589(t23967, t6492, t2031, t22550, t6495, t7032, t7025, t9231, t6486, t240, t67, t1864);
    (t23968, t23970, t23973, t23975, t23978, t23992, t23993)
}
