//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1269/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1269(t1980: f64, t4566: f64, t13296: f64, t577: f64, t116: f64, t13451: f64, t1232: f64, t5407: f64, t5380: f64, t10089: f64, t13943: f64, t3205: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t42667 = t4566 * t1980;
    let t42690 = t13296 * t577;
    let t42710 = t13451 * t116;
    let t43101 = t5407 * t1232;
    let t43602 = t5380 * t1232;
    let t43710 = t5380 * t10089;
    let t44034 = t13943 * t3205;
    (t42667, t42690, t42710, t43101, t43602, t43710, t44034)
}
