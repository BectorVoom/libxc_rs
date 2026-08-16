//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 503/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk503(t2953: f64, t2954: f64, t1005: f64, t1599: f64, t1603: f64, t2937: f64, t1027: f64, t659: f64, t126: f64, t615: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2955 = t2953 * t2954;
    let t2957 = t1005 * t1599;
    let t2958 = t2937 * t1603;
    let t2959 = t2957 * t2958;
    let t2970 = t1027 * t659;
    let t2972 = t126 * t615;
    (t2955, t2957, t2958, t2959, t2970, t2972)
}
