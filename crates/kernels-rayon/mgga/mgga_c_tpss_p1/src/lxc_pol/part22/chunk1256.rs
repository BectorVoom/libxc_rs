//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1256/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1256(t19542: f64, t20190: f64, t1639: f64, t520: f64, t5918: f64, t5745: f64, t1838: f64, t4459: f64, t18967: f64, t19554: f64, t1265: f64, t5740: f64, t6419: f64) -> (f64, f64, f64, f64, f64) {
    let t20191 = t20190 * t19542;
    let t20195 = t5918 * t1639 * t520;
    let t20196 = t5745 * t20195;
    let t20200 = t5745 * t1838 * t4459 * t520;
    let t20202 = t18967 * t19554;
    let t20206 = t5740 * t6419 * t1265;
    (t20191, t20196, t20200, t20202, t20206)
}
