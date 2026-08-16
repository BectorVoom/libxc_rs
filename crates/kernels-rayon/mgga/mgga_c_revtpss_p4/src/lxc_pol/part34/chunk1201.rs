//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1201/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1201(t3114: f64, t93751: f64, t11240: f64, t11244: f64, t7120: f64, t11627: f64, t25503: f64, t1976: f64, t27639: f64, t995: f64, t25610: f64, t3268: f64, t7143: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t93752 = t3114 * t93751;
    let t93758 = t11240 * t7120 * t11244;
    let t93789 = t11240 * t11627 * sigma0 * t11244;
    let t93793 = t11240 * t25503 * t11244;
    let t93870 = t11627 * t1976;
    let t93890 = t995 * t27639;
    let t93897 = t25610 * t27639;
    let t93920 = t7143 * t3268;
    (t93752, t93758, t93789, t93793, t93870, t93890, t93897, t93920)
}
