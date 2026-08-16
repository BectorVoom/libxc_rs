//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 633/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk633(t652: f64, t8327: f64, t1902: f64, t225: f64, t258: f64, t214: f64, t1880: f64, t1911: f64, t6571: f64) -> (f64, f64, f64, f64, f64) {
    let t8328 = t652 * t8327;
    let t8329 = 2.0_f64 * t8328;
    let t8331 = t1902 * t225 * t258;
    let t8332 = t214 * t8331;
    let t8334 = 0.16449340668482264365e-1_f64 * t1880 * t8332;
    let t8335 = t6571 * t1911;
    (t8329, t8331, t8332, t8334, t8335)
}
