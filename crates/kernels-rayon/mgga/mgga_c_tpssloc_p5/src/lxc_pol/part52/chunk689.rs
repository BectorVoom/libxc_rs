//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 689/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk689(t1343: f64, t5287: f64, t820: f64, t1352: f64, t5248: f64, t5249: f64, t120: f64, t1799: f64, t3805: f64, t1831: f64, t3866: f64, t1307: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5289 = t1343 * t820 * t5287;
    let t5293 = t5248 * t5249 * t1352;
    let t5301 = t120 * t1799;
    let t5303 = t3805 * t5301 * t1352;
    let t5306 = t3866 * t1831;
    let t5308 = t1799 * t1307;
    (t5289, t5293, t5301, t5303, t5306, t5308)
}
