//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1021/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1021(t1280: f64, t24633: f64, t1811: f64, t6628: f64, t3769: f64, t5464: f64, t6622: f64, t5332: f64, t1287: f64, t24739: f64, t24751: f64, t24704: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24964 = t1280 * t24633;
    let t24973 = t1811 * t6628;
    let t24974 = t24973 * t3769;
    let t24977 = t5464 * t6622;
    let t24978 = t5332 * t24977;
    let t24981 = t24739 * t1287;
    let t24986 = t24751 * t1287;
    let t24989 = t24704 * t1287;
    (t24964, t24973, t24974, t24978, t24981, t24986, t24989)
}
