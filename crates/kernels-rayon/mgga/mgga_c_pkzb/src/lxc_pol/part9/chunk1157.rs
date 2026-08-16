//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1157/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1157(t2667: f64, t5296: f64, t5257: f64, t6941: f64, t1721: f64, t568: f64, t1719: f64, t2639: f64, t5391: f64, t17051: f64, t175: f64, t2590: f64, t2595: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20164 = t5296 * t2667;
    let t20166 = t5257 * t6941;
    let t20168 = t1721 * t568;
    let t20195 = t2639 * t5391 * t1719;
    let t20199 = t17051 * t175;
    let t20201 = t2590 * t20199 * t2595;
    (t20164, t20166, t20168, t20195, t20199, t20201)
}
