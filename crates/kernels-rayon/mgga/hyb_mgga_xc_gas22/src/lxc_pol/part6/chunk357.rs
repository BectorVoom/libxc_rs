//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 357/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk357(t143: f64, t1252: f64, t1255: f64, t1257: f64, t1259: f64, t1261: f64, t1263: f64, t1265: f64, t1267: f64, t1270: f64, t1285: f64, t172: f64, t187: f64, t694: f64) -> f64 {
    let t144 = 0.135e1_f64 <= t143;
    let t1289 = piecewise3(t144, -t694 * t1252 / 18.0_f64 + t1255 / 240.0_f64 - t1257 / 4480.0_f64 + t1259 / 103680.0_f64 - t1261 / 2838528.0_f64 + t1263 / 89456640.0_f64 - t1265 / 0.31850496e10_f64 + t1267 / 0.1263403008e12_f64, -8.0_f64 / 3.0_f64 * t1270 * t187 - 8.0_f64 / 3.0_f64 * t172 * t1285);
    t1289
}
