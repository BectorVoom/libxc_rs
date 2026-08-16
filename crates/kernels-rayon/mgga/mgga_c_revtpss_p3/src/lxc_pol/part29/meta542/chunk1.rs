//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1878/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1878(t2439: f64, t26434: f64, t887: f64, t2471: f64, t26563: f64, t10985: f64, t26576: f64, t2062: f64, t2769: f64, t786: f64, t10997: f64, t26519: f64, t93157: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95925 = t2439 * t26434 * t887;
    let t95927 = t26563 * t2471;
    let t95930 = 0.46263278077393568556e-2_f64 * t26576 * t10985;
    let t95936 = t786 * t2062 * t2769;
    let t95937 = t95936 * t10997;
    let t95945 = t93157 * t26519;
    (t95925, t95927, t95930, t95936, t95937, t95945)
}
