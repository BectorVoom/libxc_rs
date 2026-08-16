//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1730/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1730(t1873: f64, t26004: f64, t5690: f64, t7252: f64, t1398: f64, t1903: f64, t543: f64, t1955: f64, t5710: f64, t1513: f64, t25823: f64, t665: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27955 = t26004 * t1873;
    let t27957 = t7252 * t5690;
    let t27972 = t1903 * t1398 * t543;
    let t28008 = t1955 * t5710;
    let t28034 = t25823 * t1513;
    let t28036 = t1513 * t665;
    (t27955, t27957, t27972, t28008, t28034, t28036)
}
