//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 786/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk786(t2274: f64, t50: f64, t2244: f64, t2250: f64, t22510: f64, t7251: f64, t67: f64, t1864: f64, t6509: f64, t7255: f64, t2109: f64, t22489: f64) -> (f64, f64, f64, f64) {
    let t24498 = t50 * t2274;
    let t24503 = 5.0_f64 / 18.0_f64 * t24498 * t2244 - 5.0_f64 / 6.0_f64 * t7251 * t2250 - t22510;
    let t24504 = t24503 * t67;
    let t24505 = t24504 * t1864;
    let t24508 = t7255 * t6509;
    let t24511 = t2109 * t22489;
    (t24503, t24505, t24508, t24511)
}
