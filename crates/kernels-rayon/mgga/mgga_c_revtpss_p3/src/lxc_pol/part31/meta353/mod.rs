//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta353 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1367;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1368;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta353(t14054: f64, t3992: f64, t2661: f64, t5774: f64, t72: f64, t686: f64, t3915: f64, t5711: f64, t786: f64, t1364: f64, t1357: f64, t5775: f64, t689: f64, t2470: f64, t5721: f64, t1445: f64, t5599: f64, t2435: f64, t5600: f64, t1426: f64, t1893: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14057, t14079, t14081, t14084, t14085) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1367(t14054, t3992, t2661, t5774, t72, t686, t3915, t5711, t786, t1364, t1357, t5775);
        let (t14087, t14090, t14091, t14096, t14097, t14100) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1368(t14085, t689, t2470, t5721, t3915, t1445, t5599, t2435, t5600, t1426, t1893, t786);
    (t14057, t14079, t14081, t14084, t14087, t14090, t14091, t14096, t14097, t14100)
}
