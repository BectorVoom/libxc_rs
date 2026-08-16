//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1876/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1876(t7407: f64, t93179: f64, t25365: f64, t26506: f64, t25305: f64, t95540: f64, t10115: f64, t2063: f64, t213: f64, t26473: f64, t10982: f64, t2061: f64, t9646: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95876 = t93179 * t7407;
    let t95888 = t25365 * t26506;
    let t95891 = 0.91399340044406952588e-2_f64 * t25305 * t95540;
    let t95893 = 0.11044544084478153697e-3_f64 * t10115 * t2063;
    let t95894 = t213 * t26473;
    let t95899 = 0.19637199382202157274e-3_f64 * t9646 * t2061 * t10982;
    (t95876, t95888, t95891, t95893, t95894, t95899)
}
