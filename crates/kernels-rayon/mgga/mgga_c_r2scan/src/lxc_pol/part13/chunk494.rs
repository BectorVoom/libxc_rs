//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 494/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk494(t2214: f64, t530: f64, t514: f64, t1632: f64, t481: f64, t551: f64, t566: f64, t489: f64, t525: f64, t524: f64, t1543: f64, t506: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2215 = t2214 * t530;
    let t2216 = t514 * t2215;
    let t2218 = t1632 * t481;
    let t2219 = t551 * t2218;
    let t2220 = t566 * t2219;
    let t2222 = t525 * t489;
    let t2223 = t524 * t2222;
    let t2224 = t506 * t1543;
    (t2215, t2216, t2218, t2219, t2220, t2222, t2223, t2224)
}
