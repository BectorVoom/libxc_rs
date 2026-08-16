//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 873/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk873(t5371: f64, t8326: f64, t1458: f64, t3941: f64, t7042: f64, t7468: f64, t1874: f64, t27188: f64, t1441: f64, t1873: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33191 = t5371 * t8326;
    let t33192 = 0.135e2_f64 * t33191;
    let t33193 = t8326 * t1458;
    let t33194 = t3941 * t33193;
    let t33195 = 27.0_f64 * t33194;
    let t33199 = 2.0_f64 * t7042 * t7468;
    let t33208 = 2.0_f64 * t27188 * t1874;
    let t33211 = t1441 * t1873;
    (t33191, t33192, t33193, t33194, t33195, t33199, t33208, t33211)
}
