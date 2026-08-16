//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 926/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk926(t143: f64, t8334: f64, t1255: f64, t1257: f64, t1259: f64, t1261: f64, t1263: f64, t1265: f64, t1267: f64, t2042: f64, t694: f64, t712: f64, t716: f64, t720: f64, t724: f64, t728: f64, t732: f64, t736: f64, t8267: f64) -> (f64, f64) {
    let t145 = 0.135e1_f64 < t143;
    let t8335 = piecewise3(t145, t8334, 0.0_f64);
    let t8352 = t8267 * t2042 / 412876800.0_f64 - 2.0_f64 / 3.0_f64 * t1255 * t2042 + t1257 * t2042 / 8.0_f64 - t1259 * t2042 / 80.0_f64 + t1261 * t2042 / 1152.0_f64 - t1263 * t2042 / 21504.0_f64 + t1265 * t2042 / 491520.0_f64 - t1267 * t2042 / 13271040.0_f64 - t732 * t8335 / 0.31850496e10_f64 + t736 * t8335 / 0.1263403008e12_f64 - t694 * t8335 / 18.0_f64 + t712 * t8335 / 240.0_f64 - t716 * t8335 / 4480.0_f64 + t720 * t8335 / 103680.0_f64 - t724 * t8335 / 2838528.0_f64 + t728 * t8335 / 89456640.0_f64;
    (t8335, t8352)
}
