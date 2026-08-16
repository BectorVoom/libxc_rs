//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1240/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1240(t22633: f64, t26403: f64, t3807: f64, t6976: f64, t114057: f64, t114060: f64, t22751: f64, t32741: f64, t1338: f64, t32726: f64, t114069: f64, t1799: f64, t6637: f64, t6888: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120467 = 0.3289868133696452873e-1_f64 * t22633 * t6976 * t26403 * t3807;
    let t120468 = 0.76763589786250567036e-1_f64 * t114057;
    let t120469 = 0.16449340668482264365e-1_f64 * t114060;
    let t120470 = t22751 * t32741;
    let t120471 = 0.76763589786250567037e-1_f64 * t120470;
    let t120475 = t1338 * t32726;
    let t120483 = 0.3289868133696452873e-1_f64 * t6888 * t6637 * t114069 * t1799;
    (t120467, t120468, t120469, t120471, t120475, t120483)
}
