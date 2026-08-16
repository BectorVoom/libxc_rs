//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta755 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2537;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2538;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta755(t71371: f64, t71389: f64, t1107: f64, t63332: f64, t63334: f64, t63336: f64, t63886: f64, t63888: f64, t63893: f64, t71124: f64, t71130: f64, t71135: f64, t71140: f64, t71142: f64, t136: f64, t43761: f64, t71164: f64, t1100: f64, t1113: f64, t71148: f64, t21794: f64, t699: f64, t11219: f64, t71158: f64, t71133: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t71390, t71391, t71396) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2537(t71371, t71389, t1107, t63332, t63334, t63336, t63886, t63888, t63893, t71124, t71130, t71135, t71140, t71142);
        let (t71400, t71403, t71406, t71408, t71411, t71414) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2538(t136, t43761, t71164, t1100, t71390, t1113, t71148, t21794, t699, t11219, t71158, t71133);
    (t71391, t71396, t71400, t71403, t71406, t71408, t71411, t71414)
}
