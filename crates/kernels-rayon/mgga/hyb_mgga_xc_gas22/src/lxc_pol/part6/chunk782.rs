//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 782/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk782(t4251: f64, t950: f64, t2496: f64, t4247: f64, t957: f64, t1392: f64, t238: f64, t242: f64, t343: f64, t4234: f64, t2493: f64, t2503: f64, t3461: f64, t3503: f64, t4236: f64, t4248: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4252 = t950 * t4251;
    let t4256 = t2496 * t4247;
    let t4258 = t957 * t4251;
    let t4261 = t1392 * t1392;
    let t4263 = t238 * t242 * t4261;
    let t4265 = t343 * t4234;
    let t4267 = t238 * t242 * t4265;
    let t4269 = -0.9494625e0_f64 * t4248 + 0.1898925e1_f64 * t4252 + t2493 - 0.59793333333333333334e0_f64 * t3461 + 0.8969e0_f64 * t4236 + 0.15358125e0_f64 * t4256 + 0.3071625e0_f64 * t4258 + t2503 - 0.32862666666666666666e0_f64 * t3503 + 0.24647e0_f64 * t4263 + 0.24647e0_f64 * t4267;
    (t4252, t4256, t4258, t4261, t4263, t4265, t4267, t4269)
}
