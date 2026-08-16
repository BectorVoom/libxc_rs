//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1878/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1878(t1294: f64, t7644: f64, t7652: f64, t1204: f64, t2142: f64, t1209: f64, t26936: f64) -> (f64, f64, f64) {
    let t27015 = t7652 * t7644 * t1294;
    let t27020 = t1204 * t2142;
    let t27025 = t1209 * t26936;
    (t27015, t27020, t27025)
}
