//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1168/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1168(t10856: f64, t9423: f64, t11643: f64, t25983: f64, t261: f64, t3304: f64, t9476: f64, t37982: f64, t9373: f64, t11654: f64, t7601: f64, t10743: f64, t3198: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43217 = t10856 * t9423;
    let t43219 = t25983 * t11643;
    let t43225 = t3304 * t261 * t9476;
    let t43230 = t37982 * t9373;
    let t43232 = t7601 * t11654;
    let t43234 = t10743 * t3198;
    (t43217, t43219, t43225, t43230, t43232, t43234)
}
