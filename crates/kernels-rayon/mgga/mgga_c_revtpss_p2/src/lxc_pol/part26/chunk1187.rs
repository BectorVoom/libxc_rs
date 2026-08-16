//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1187/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1187(t26502: f64, t786: f64, t789: f64, t93314: f64, t95854: f64, t7407: f64, t93179: f64, t25365: f64, t26506: f64, t25305: f64, t95540: f64, t10115: f64, t2063: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95866 = t786 * t26502 * t789;
    let t95872 = t93314 * t95854;
    let t95876 = t93179 * t7407;
    let t95888 = t25365 * t26506;
    let t95891 = 0.91399340044406952588e-2_f64 * t25305 * t95540;
    let t95893 = 0.11044544084478153697e-3_f64 * t10115 * t2063;
    (t95866, t95872, t95876, t95888, t95891, t95893)
}
