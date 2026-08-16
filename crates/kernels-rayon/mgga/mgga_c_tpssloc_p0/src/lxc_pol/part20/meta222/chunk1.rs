//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1296/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1296(t1864: f64, t2250: f64, t2244: f64, t628: f64, t584: f64, t9212: f64) -> (f64, f64, f64, f64) {
    let t9248 = t1864 * t2250;
    let t9251 = t2244 * t628;
    let t9256 = t584 - t9212;
    let t9257 = 6.0_f64 * t9256;
    (t9248, t9251, t9256, t9257)
}
