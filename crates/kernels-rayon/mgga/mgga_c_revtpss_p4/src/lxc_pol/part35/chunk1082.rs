//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1082/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1082(t1955: f64, t6888: f64, t1882: f64, t1903: f64, t543: f64, t1868: f64, t1907: f64, t1501: f64, t1518: f64) -> (f64, f64, f64, f64) {
    let t30071 = t1955 * t6888;
    let t30105 = t1903 * t1882 * t543;
    let t30122 = t1868 * t1907;
    let t30138 = t1501 * t1518;
    (t30071, t30105, t30122, t30138)
}
