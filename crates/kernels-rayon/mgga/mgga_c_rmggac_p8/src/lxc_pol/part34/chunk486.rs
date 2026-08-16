//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 486/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk486(t3046: f64, t3810: f64, t2566: f64, t2048: f64, t637: f64, t854: f64, t1322: f64) -> (f64, f64, f64, f64) {
    let t13931 = t3810 * t3046;
    let t13932 = t13931 * t2566;
    let t13935 = t854 * t2048 * t637;
    let t13937 = t3810 * t1322;
    (t13931, t13932, t13935, t13937)
}
