//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1952/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1952(t18498: f64, t27763: f64, t106554: f64, t27799: f64, t18838: f64, t33: f64, t1353: f64, t6922: f64, t30105: f64, t689: f64, t1882: f64, t543: f64, t5774: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t108033 = t27763 * t18498;
    let t108036 = t27799 * t106554;
    let t108043 = t33 * t18838;
    let t108126 = t6922 * t1353;
    let t108138 = t30105 * t689;
    let t108178 = t5774 * t1882 * t543;
    (t108033, t108036, t108043, t108126, t108138, t108178)
}
