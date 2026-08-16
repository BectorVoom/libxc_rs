//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1189/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1189(t116: f64, t5815: f64, t1338: f64, t623: f64, t3537: f64, t94: f64, t6076: f64, t619: f64, t77: f64, t1317: f64, t1679: f64, t1290: f64, t1981: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19040 = t116 * t5815;
    let t19305 = t623 * t1338;
    let t19308 = t94 * t3537;
    let t19342 = t77 * t6076 * t619;
    let t19345 = t1679 * t1317;
    let t19349 = t1981 * t1290;
    (t19040, t19305, t19308, t19342, t19345, t19349)
}
