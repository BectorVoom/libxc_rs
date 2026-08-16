//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1214/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1214(t1209: f64, t12722: f64, t1248: f64, t3584: f64, t1287: f64, t12233: f64, t12240: f64, t12242: f64, t12245: f64, t12251: f64, t12360: f64, t12363: f64, t12573: f64, t12575: f64, t12577: f64, t12598: f64) -> (f64, f64, f64, f64) {
    let t12723 = t1209 * t12722;
    let t12726 = t3584 * t1248;
    let t12727 = t12726 * t1287;
    let t12730 = t12240 + t12242 + t12245 - t12251 + t12360 + t12233 - t12598 - t12575 - t12577 - t12573 - t12363;
    (t12723, t12726, t12727, t12730)
}
