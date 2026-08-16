//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1078/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1078(t1576: f64, t546: f64, t2079: f64, t545: f64, t25851: f64, t512: f64, t6156: f64, t10757: f64, t776: f64, t261: f64, t6499: f64, t7614: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t37965 = t546 * t1576;
    let t37972 = t545 * t2079;
    let t37982 = t512 * t25851;
    let t37983 = t37982 * t6156;
    let t37985 = t776 * t10757;
    let t37988 = t7614 * t261 * t6499;
    (t37965, t37972, t37982, t37983, t37985, t37988)
}
