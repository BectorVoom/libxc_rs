//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1069/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1069(t1591: f64, t37754: f64, t10768: f64, t6214: f64, t10710: f64, t20294: f64, t10697: f64, t10780: f64, t2127: f64, t2214: f64, t503: f64, t6156: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t37755 = t1591 * t37754;
    let t37759 = t10768 * t6214;
    let t37762 = t10768 * t10710 * t20294;
    let t37764 = t10697 * t10780;
    let t37765 = t37764 * t2127;
    let t37769 = t503 * t2214;
    let t37770 = t37769 * t6156;
    (t37755, t37759, t37762, t37764, t37765, t37769, t37770)
}
