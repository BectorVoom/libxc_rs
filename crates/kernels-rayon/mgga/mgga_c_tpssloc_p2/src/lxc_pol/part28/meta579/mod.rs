//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta579 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1863;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1864;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta579(t13193: f64, t6621: f64, t13198: f64, t23097: f64, t232: f64, t46565: f64, t815: f64, t46644: f64, t25135: f64, t838: f64, t2693: f64, t7503: f64, t25132: f64, t81882: f64, t6604: f64, t81968: f64, t13184: f64, t841: f64, t23083: f64, t25123: f64, t13191: f64, t25119: f64, t1878: f64, t81982: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87389, t87391, t87395, t87399, t87401, t87403) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1863(t13193, t6621, t13198, t23097, t232, t46565, t815, t46644, t25135, t838, t2693, t7503);
        let (t87405, t87409, t87411, t87418, t87420) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1864(t25132, t81882, t6604, t81968, t13184, t841, t23083, t25123, t13191, t25119, t1878, t81982);
    (t87389, t87391, t87395, t87399, t87401, t87403, t87405, t87409, t87411, t87418, t87420)
}
