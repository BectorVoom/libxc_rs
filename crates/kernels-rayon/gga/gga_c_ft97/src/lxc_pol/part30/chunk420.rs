//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 420/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk420(t2568: f64, t6930: f64, t242: f64, t6181: f64, t6184: f64, t6881: f64, t6885: f64, t6889: f64, t6893: f64, t6897: f64, t6901: f64, t6905: f64) -> (f64, f64) {
    let t6931 = t2568 * t6930;
    let t6932 = t242 * t6931;
    let t6940 = t6881 / 4.0_f64 + t6181 + t6885 / 6.0_f64 + t6889 - t6893 / 2.0_f64 + t6184 + t6897 / 3.0_f64 + 2.0_f64 * t6901 - t6905;
    (t6932, t6940)
}
