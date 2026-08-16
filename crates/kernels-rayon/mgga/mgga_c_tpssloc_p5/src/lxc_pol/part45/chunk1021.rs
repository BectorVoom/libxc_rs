//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1021/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1021(t22724: f64, t31594: f64, t115332: f64, t1985: f64, t6907: f64, t2085: f64, t213: f64, t225: f64, t22633: f64, t22637: f64, t22642: f64, t22643: f64, t8621: f64) -> (f64, f64, f64, f64) {
    let t115539 = t22724 * t31594;
    let t115540 = 0.26044789391763585244e-1_f64 * t115539;
    let t115542 = t1985 * t115332 * t6907;
    let t115545 = t213 * t2085 * t225;
    let t115547 = t22633 * t115545 * t22637;
    let t115550 = t22642 * t22643 * t8621;
    (t115540, t115542, t115547, t115550)
}
