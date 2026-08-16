//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1142/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1142(t102204: f64, t94589: f64, t2470: f64, t28779: f64, t25895: f64, t94771: f64, t2435: f64, t28902: f64, t2453: f64, t3908: f64, t8086: f64, t25878: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t102205 = t94589 * t102204;
    let t102218 = t28779 * t2470;
    let t102219 = t25895 * t102218;
    let t102225 = t94771 * t102204;
    let t102249 = t2435 * t28902;
    let t102266 = t2453 * t8086 * t3908;
    let t102293 = t25878 * t102218;
    (t102205, t102219, t102225, t102249, t102266, t102293)
}
