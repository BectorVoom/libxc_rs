//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 703/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk703(t1307: f64, t1377: f64, t1385: f64, t22635: f64, t22633: f64, t154: f64, t835: f64, t3748: f64) -> (f64, f64, f64, f64) {
    let t22637 = t1377 * t1307 * t1385;
    let t22638 = t22635 * t22637;
    let t22639 = t22633 * t22638;
    let t22641 = t835 * t154;
    let t22642 = t22641 * t3748;
    (t22637, t22639, t22641, t22642)
}
