//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1410/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1410(t34878: f64, t34881: f64, t34886: f64, t34891: f64, t34897: f64, t34873: f64, t34876: f64, t34884: f64, t34889: f64, t34894: f64, t34900: f64, t34909: f64) -> (f64, f64) {
    let t37111 = 0.11196959561581759181e-6_f64 * t34878;
    let t37112 = 0.16555927416768851825e-5_f64 * t34881;
    let t37114 = 0.28137654660407340486e-7_f64 * t34886;
    let t37116 = 0.41268560168597432712e-4_f64 * t34891;
    let t37118 = 0.2845640240200497334e-7_f64 * t34897;
    let t37120 = -0.3623181683912940217e-6_f64 * t34873 - 0.44979384805509945071e-8_f64 * t34876 + t37111 + t37112 - 0.19666550313313802087e-7_f64 * t34884 + t37114 - 0.52389984474979915324e-8_f64 * t34889 - t37116 + 0.93149392396514289451e-9_f64 * t34894 + t37118 - 0.505954834707648426e-7_f64 * t34900;
    let t37124 = 0.16867947048611111112e-5_f64 * t34909;
    (t37120, t37124)
}
