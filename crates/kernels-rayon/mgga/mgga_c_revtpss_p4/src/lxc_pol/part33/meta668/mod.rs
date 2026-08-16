//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta668 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2193;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2194;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta668(t27126: f64, t7735: f64, t1497: f64, t4237: f64, t77: f64, t1493: f64, t4241: f64, t5872: f64, t640: f64, t21809: f64, t84: f64, t4186: f64, t2242: f64, t5826: f64, t19680: f64, t603: f64, t21663: f64, t607: f64, t5868: f64, t644: f64, t13269: f64, t1470: f64, t4173: f64, t4181: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t108727, t108733, t108737, t108745, t108749, t108759) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2193(t27126, t7735, t1497, t4237, t77, t1493, t4241, t5872, t640, t21809, t84, t4186);
        let (t108762, t108765, t108769, t108792, t108807, t108810) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2194(t2242, t5826, t19680, t603, t21663, t607, t5868, t644, t77, t13269, t1470, t4173, t4181);
    (t108727, t108733, t108737, t108745, t108749, t108759, t108762, t108765, t108769, t108792, t108807, t108810)
}
