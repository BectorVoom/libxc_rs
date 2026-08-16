//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1428/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1428(t12213: f64, t1710: f64, t38961: f64, t723: f64, t12218: f64, t12220: f64, t12224: f64, t12228: f64, t12256: f64, t1445: f64, t1457: f64, t2049: f64, t2087: f64, t2197: f64, t2615: f64, t326: f64, t33228: f64, t33231: f64, t33238: f64, t33240: f64, t33246: f64, t33248: f64, t33253: f64, t33255: f64, t39044: f64, t5782: f64, t7736: f64, t833: f64) -> (f64, f64, f64) {
    let t39091 = t12213 * t1710;
    let t39095 = t38961 * t723;
    let t39101 = t33228 - t33231 - t33238 - t33240 - 0.69017266717057349418e1_f64 * t2087 * t1445 * t12218 * t1710 - 0.13803453343411469884e2_f64 * t5782 * t12220 + 0.92023022289409799224e1_f64 * t2615 * t326 * t39044 + t33246 - 0.10725146985555128001e1_f64 * t12256 * t7736 + t33248 - 0.21450293971110256002e1_f64 * t2049 * t1457 * t12224 + 0.11502877786176224903e2_f64 * t833 * t1445 * t39091 + 0.23005755572352449806e2_f64 * t833 * t1445 * t39095 + 0.23005755572352449806e2_f64 * t2197 * t12228 - t33253 - t33255;
    (t39091, t39095, t39101)
}
