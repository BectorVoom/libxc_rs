//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1342/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1342(t114057: f64, t114060: f64, t22751: f64, t32741: f64, t114069: f64, t1799: f64, t6637: f64, t6888: f64, t31193: f64, t5187: f64, t22892: f64, t22893: f64, t32740: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120468 = 0.76763589786250567036e-1_f64 * t114057;
    let t120469 = 0.16449340668482264365e-1_f64 * t114060;
    let t120470 = t22751 * t32741;
    let t120471 = 0.76763589786250567037e-1_f64 * t120470;
    let t120483 = 0.3289868133696452873e-1_f64 * t6888 * t6637 * t114069 * t1799;
    let t120487 = 0.3289868133696452873e-1_f64 * t6888 * t6637 * t31193 * t5187;
    let t120490 = t22892 * t22893 * t32740;
    (t120468, t120469, t120471, t120483, t120487, t120490)
}
