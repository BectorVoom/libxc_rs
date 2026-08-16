//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 694/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk694(t4459: f64, t520: f64, t1224: f64, t774: f64, t1233: f64, t4416: f64, t4415: f64, t125: f64, t1625: f64, t3273: f64, t1646: f64, t3342: f64) -> (f64, f64, f64, f64, f64) {
    let t4460 = t4459 * t520;
    let t4462 = t1224 * t774 * t4460;
    let t4465 = t4416 * t1233;
    let t4466 = t4415 * t4465;
    let t4471 = t125 * t1625;
    let t4472 = t4471 * t1233;
    let t4473 = t3273 * t4472;
    let t4476 = t3342 * t1646;
    (t4460, t4462, t4466, t4473, t4476)
}
