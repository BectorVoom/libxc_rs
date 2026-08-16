//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta580 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1999;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2000;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta580(t11874: f64, t27492: f64, t11988: f64, t7132: f64, t11997: f64, t25503: f64, t3141: f64, t1052: f64, t3089: f64, t1087: f64, t11970: f64, t1973: f64, sigma0: f64, t3201: f64, t7126: f64, t7114: f64, t1024: f64, t25576: f64, t7120: f64, t11858: f64, t11926: f64, t25516: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t93548, t93555, t93567, t93596, t93597, t93611) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1999(t11874, t27492, t11988, t7132, t11997, t25503, t3141, t1052, t3089, t1087, t11970, t1973, sigma0);
        let (t93618, t93622, t93646, t93655, t93658, t93667) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2000(t3201, t7126, t7114, t1024, t25576, t11997, t3141, t7120, t11858, t27492, t11926, t25516);
    (t93548, t93555, t93567, t93596, t93597, t93611, t93618, t93622, t93646, t93655, t93658, t93667)
}
