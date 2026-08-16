//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 270/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk270(t916: f64, t918: f64, t902: f64, t273: f64, t240: f64, t696: f64, t281: f64, t283: f64, t346: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t919 = t916 * t918;
    let t921 = 0.29896666666666666667e0_f64 * t902;
    let t923 = f64::sqrt(t273);
    let t924 = t923 * t918;
    let t926 = t696 * t240;
    let t928 = t281 * t926 * t283;
    let t929 = 0.82156666666666666667e-1_f64 * t928;
    let t930 = t240 * t346;
    (t919, t921, t923, t924, t926, t928, t929, t930)
}
