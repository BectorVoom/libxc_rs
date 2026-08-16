//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1232/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1232(t6589: f64, t68: f64, t13151: f64, t1891: f64, t225: f64, t228: f64, t230: f64, t2379: f64, t2553: f64, t2667: f64, t2671: f64, t2672: f64, t2675: f64, t40848: f64, t40972: f64, t40977: f64, t41241: f64, t41242: f64, t41244: f64, t41245: f64, t41248: f64, t41249: f64, t41263: f64, t41297: f64, t4225: f64, t822: f64, t824: f64, t825: f64, t9516: f64, t9938: f64, t9947: f64, t9950: f64, t9951: f64, t9954: f64) -> f64 {
    let t41315 = t68 * t6589;
    let t41332 = -(t41241 + t41242 + t41244 + t41245 + t41248 + t41249 + t41263 + t41297) * t225 * t230 + 12.0_f64 * t9938 * t825 - 72.0_f64 * t2667 * t2672 + 18.0_f64 * t2667 * t2675 + 240.0_f64 * t822 * t9947 - 144.0_f64 * t13151 * t9951 + 12.0_f64 * t822 * t9954 - 360.0_f64 * t228 * t41315 * t40972 + 360.0_f64 * t4225 * t1891 * t2379 * t2553 - 36.0_f64 * t228 * t2671 * t40977 - 48.0_f64 * t4225 * t9950 * t9516 + 3.0_f64 * t228 * t824 * t40848;
    t41332
}
