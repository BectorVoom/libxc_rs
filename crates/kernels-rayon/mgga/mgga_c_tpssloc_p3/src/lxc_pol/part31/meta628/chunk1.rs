//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1886/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1886(t1369: f64, t97265: f64, t1339: f64, t1824: f64, t22827: f64, t5187: f64, t550: f64, t74677: f64, t1307: f64, t3788: f64, t6388: f64, t22783: f64, t6427: f64) -> (f64, f64, f64, f64, f64) {
    let t97266 = t97265 * t1369;
    let t97273 = t22827 * t1339 * t5187 * t1824 * t550;
    let t97277 = t22827 * t1339 * t74677 * t550;
    let t97281 = t22827 * t3788 * t6388 * t1307;
    let t97283 = t22783 * t6427;
    (t97266, t97273, t97277, t97281, t97283)
}
