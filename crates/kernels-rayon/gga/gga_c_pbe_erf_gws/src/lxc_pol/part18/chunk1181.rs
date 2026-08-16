//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1181/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1181(t15204: f64, t3983: f64, t1192: f64, t3703: f64, t2376: f64, t2409: f64, t4155: f64, t8589: f64, t2503: f64, t4127: f64, t3863: f64, t4039: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15205 = t3983 * t15204;
    let t15207 = t1192 * t3703;
    let t15209 = t2409 * t2376 * t15207;
    let t15213 = t2409 * t8589 * t4155;
    let t15216 = t4127 * t2503;
    let t15218 = t4039 * t3863;
    (t15205, t15207, t15209, t15213, t15216, t15218)
}
