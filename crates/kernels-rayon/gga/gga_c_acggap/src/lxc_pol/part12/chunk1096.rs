//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1096/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1096(t30268: f64, t8783: f64, t1479: f64, t429: f64, t1980: f64, t7476: f64, t1089: f64, t15897: f64, t2288: f64, t598: f64, t1988: f64, t8486: f64) -> (f64, f64, f64, f64, f64) {
    let t35496 = t30268 * t8783;
    let t35500 = t429 * t1479;
    let t35502 = t1980 * t7476 * t35500;
    let t35511 = t598 * t1089 * t15897 * t2288;
    let t35513 = t1988 * t8486;
    (t35496, t35500, t35502, t35511, t35513)
}
