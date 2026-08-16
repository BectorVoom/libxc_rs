//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1315/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1315(t113138: f64, t113160: f64, t113267: f64, t113291: f64, t113320: f64, t113351: f64, t113380: f64, t113412: f64, t892: f64, t1468: f64, t5962: f64, t23421: f64, t30: f64) -> (f64, f64, f64, f64) {
    let t113415 = t113138 + t113160 + t113267 + t113291 + t113320 + t113351 + t113380 + t113412;
    let t113416 = t113415 * t892;
    let t113420 = t1468 * t5962;
    let t113424 = t30 * t23421;
    (t113415, t113416, t113420, t113424)
}
