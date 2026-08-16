//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta290 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1295;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1296;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta290(t207: f64, t795: f64, t9580: f64, t2690: f64, t841: f64, t812: f64, t849: f64, t241: f64, t6589: f64, t67: f64, t2632: f64, t776: f64, t815: f64, t836: f64, t2617: f64, t2642: f64, t1891: f64, t246: f64, t2628: f64, t835: f64, t831: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9583, t9601, t9602, t9607, t9627) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1295(t207, t795, t9580, t2690, t841, t812, t849, t241, t6589, t67, t2632, t776);
        let (t9638, t9642, t9645, t9646, t9667, t9671, t9672) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1296(t815, t836, t812, t2617, t2642, t1891, t67, t246, t2628, t835, t2690, t831);
    (t9583, t9601, t9602, t9607, t9627, t9638, t9642, t9645, t9646, t9667, t9671, t9672)
}
