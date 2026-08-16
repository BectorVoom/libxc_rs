//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 595/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk595(t10012: f64, t723: f64, t9438: f64, t2684: f64, t10007: f64, t701: f64, t2615: f64, t2628: f64, t2673: f64, t7442: f64, t787: f64, t2563: f64, t900: f64) -> (f64, f64, f64, f64, f64) {
    let t10013 = t10012 * t723;
    let t10014 = t9438 * t10013;
    let t10015 = t2684 * t10014;
    let t10017 = t10007 * t701;
    let t10018 = t9438 * t10017;
    let t10019 = t2615 * t10018;
    let t10022 = 0.59584149919750711116e-1_f64 * t2673 * t2628;
    let t10023 = t787 * t7442;
    let t10024 = t900 * t2563;
    (t10015, t10019, t10022, t10023, t10024)
}
