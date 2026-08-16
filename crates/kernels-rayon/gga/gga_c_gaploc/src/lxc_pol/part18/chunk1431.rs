//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1431/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1431(t35228: f64, t10434: f64, t1391: f64, t2487: f64, t1441: f64, t31412: f64, t31414: f64, t31417: f64, t34567: f64, t35192: f64, t35199: f64, t35201: f64, t35206: f64, t35209: f64, t35211: f64, t35214: f64, t35219: f64, t35220: f64, t35226: f64, t590: f64, t6710: f64, t6711: f64) -> f64 {
    let t35229 = 0.51123901271894332902e0_f64 * t35228;
    let t35231 = t2487 * t1391 * t10434;
    let t35232 = 0.2698205900461089792e0_f64 * t35231;
    let t35233 = -t35192 - 0.23005755572352449806e2_f64 * t6710 * t6711 * t34567 - t35199 - t35201 - t35206 - t35209 + t35211 - t35214 - t35219 + 0.2044956050875773316e1_f64 * t1441 * t35220 * t590 - t31412 - t31414 - t31417 - t35226 - t35229 + t35232;
    t35233
}
