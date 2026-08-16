//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1193/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1193(t226: f64, t782: f64, t818: f64, t2157: f64, t811: f64, t2433: f64, t30: f64, t580: f64, t821: f64, t2428: f64, t2116: f64, t33: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18009 = t818 * t782 * t226;
    let t18021 = t811 * t2157;
    let t18053 = t30 * t2433;
    let t18056 = t580 * t821;
    let t18059 = t30 * t2428;
    let t18239 = t33 * t2116;
    (t18009, t18021, t18053, t18056, t18059, t18239)
}
