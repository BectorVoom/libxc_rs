//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 628/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk628(t1825: f64, t7208: f64, t553: f64, t7918: f64, t1336: f64, t1814: f64, t2089: f64, t544: f64, t7202: f64, t7204: f64, t7734: f64, t7738: f64, t7742: f64) -> (f64, f64, f64) {
    let t7932 = t7208 * t1825;
    let t7934 = t553 * t7918;
    let t7936 = -t7202 - 0.3289868133696452873e-1_f64 * t7734 - t7204 - 0.16449340668482264365e-1_f64 * t7738 + 0.16449340668482264365e-1_f64 * t7742 + t1814 * t2089 - t1336 * t7932 + t544 * t7934;
    (t7932, t7934, t7936)
}
