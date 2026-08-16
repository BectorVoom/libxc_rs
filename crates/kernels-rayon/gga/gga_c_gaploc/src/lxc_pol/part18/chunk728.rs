//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 728/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk728(t4820: f64, t6510: f64, t107: f64, t2299: f64, t1415: f64, t1359: f64, t2405: f64, t544: f64, t6520: f64, t4376: f64, t901: f64, t1328: f64, t6508: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6825 = t4820 * t6510;
    let t6830 = t2299 * t107;
    let t6831 = t1415 * t6830;
    let t6834 = t1359 * t2405;
    let t6835 = t544 * t6834;
    let t6838 = t4820 * t6520;
    let t6841 = t4376 * t901;
    let t6843 = t6508 * t1328;
    (t6825, t6831, t6834, t6835, t6838, t6841, t6843)
}
