//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 689/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk689(t5: f64, t109: f64, t1860: f64, t2032: f64, t7026: f64, t7034: f64, t7428: f64, t7432: f64, t7435: f64, t7782: f64, t112: f64, t1774: f64, t2039: f64, t7053: f64, t7464: f64) -> (f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t110 = 1.0_f64 < t109;
    let t7786 = piecewise3(t8, 0.0_f64, t7428 * t2032 / 3.0_f64 - 5.0_f64 / 3.0_f64 * t7026 * t7432 - 2.0_f64 / 3.0_f64 * t7435 * t2032 - t7034 + t1860 * t7782 / 3.0_f64);
    let t7787 = t7786 * t112;
    let t7796 = t1774 * t2039;
    let t7801 = piecewise3(t110, 0.0_f64, -t7053 - t7464 / 4.0_f64);
    (t7786, t7787, t7796, t7801)
}
