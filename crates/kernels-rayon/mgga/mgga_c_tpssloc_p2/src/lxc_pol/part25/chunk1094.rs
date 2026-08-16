//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1094/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1094(t1338: f64, t225: f64, t236: f64, t22828: f64, t80853: f64, t22783: f64, t3872: f64, t12353: f64, t6952: f64, t22788: f64, t1336: f64, t2690: f64, t6950: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t80854 = t225 * t1338;
    let t80855 = t80854 * t236;
    let t80857 = t80853 * t80855 * t22828;
    let t80859 = t22783 * t3872;
    let t80861 = t6952 * t12353;
    let t80863 = t22788 * t3872;
    let t80866 = t1336 * t6950 * t2690;
    (t80854, t80857, t80859, t80861, t80863, t80866)
}
