//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1305/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1305(t3188: f64, t708: f64, t10267: f64, t10270: f64, t10275: f64, t10278: f64, t10331: f64, t1255: f64, t1257: f64, t1259: f64, t1261: f64, t1263: f64, t1265: f64, t1267: f64, t151: f64, t163: f64, t166: f64, t169: f64, t2070: f64, t3201: f64, t3206: f64, t8267: f64, t8335: f64) -> f64 {
    let t28439 = t708 * t3188;
    let t28456 = t10267 * t2070 / 640.0_f64 + t10270 * t2070 / 1152.0_f64 - t3201 * t8335 / 5760.0_f64 - t10275 * t2070 / 11520.0_f64 - t10278 * t2070 / 21504.0_f64 + t3206 * t8335 / 129024.0_f64 + t151 * t10331 * t708 / 3.0_f64 + t163 * t10331 * t708 / 129024.0_f64 - t166 * t10331 * t708 / 3440640.0_f64 + t169 * t10331 * t708 / 0.10616832e9_f64 + t1265 * t28439 / 122880.0_f64 - t1267 * t28439 / 3317760.0_f64 + t8267 * t28439 / 103219200.0_f64 - 8.0_f64 / 3.0_f64 * t1255 * t28439 + t1257 * t28439 / 2.0_f64 - t1259 * t28439 / 20.0_f64 + t1261 * t28439 / 288.0_f64 - t1263 * t28439 / 5376.0_f64;
    t28456
}
