//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 686/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk686(t28885: f64, t28897: f64, t28911: f64, t28922: f64, t871: f64, t1501: f64, t4299: f64, t2843: f64, t25413: f64, t4255: f64, t25412: f64, t1477: f64, t683: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28924 = t28885 + t28897 + t28911 + t28922;
    let t28925 = t871 * t28924;
    let t28930 = t1501 * t4299;
    let t28931 = t2843 * t28930;
    let t28934 = t25413 * t4255;
    let t28935 = t25412 * t28934;
    let t28938 = t683 * t1477;
    (t28924, t28925, t28930, t28931, t28934, t28935, t28938)
}
