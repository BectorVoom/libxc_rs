//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1445/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1445(t1043: f64, t12050: f64, t357: f64, t19450: f64, t6244: f64, t999: f64, t1082: f64, t6234: f64, t993: f64, t225: f64, t18902: f64, t19025: f64, t19027: f64, t19029: f64, t19031: f64, t19048: f64, t19051: f64, t19053: f64, t19055: f64, t19058: f64, t19060: f64, t19062: f64, t19079: f64, t19081: f64, t19084: f64, t19130: f64, t19132: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19452 = t12050 * t1043 * t357;
    let t19453 = t19450 * t19452;
    let t19456 = t6244 * t999;
    let t19457 = t1082 * t19456;
    let t19462 = t6234 * t993;
    let t19463 = t19462 * t225;
    let t19466 = -t18902 - t19025 - t19027 - t19029 + t19031 + t19048 - t19051 - t19053 + t19055 + t19058 + t19060 + t19062 - t19079 - t19081 - t19084 + t19130 + t19132;
    (t19453, t19456, t19457, t19462, t19463, t19466)
}
