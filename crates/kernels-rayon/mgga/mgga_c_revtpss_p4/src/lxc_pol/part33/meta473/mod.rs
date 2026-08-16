//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta473 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1721;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1722;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1723;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta473(t13652: f64, t177: f64, t6800: f64, t762: f64, t13666: f64, t13668: f64, t9858: f64, t9861: f64, t13887: f64, t13664: f64, t13682: f64, t13683: f64, t9524: f64, t9542: f64, t9588: f64, t9854: f64, t9865: f64, t9868: f64, t22190: f64, t22203: f64, t22210: f64, t225: f64, t1877: f64, t73: f64, t4010: f64, t6836: f64, t1353: f64, t5591: f64, t5651: f64, t1412: f64, t6816: f64, t1394: f64, t21969: f64, t1392: f64, t1395: f64, t1879: f64, t539: f64, t541: f64, t5644: f64, t5650: f64, t5652: f64, t5655: f64, t6832: f64, t6837: f64, t6840: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22211, t22214, t22215, t22216, t22217, t22218, t22219, t22220) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1721(t13652, t177, t6800, t762, t13666, t13668, t9858, t9861, t13887, t13664, t13682, t13683, t9524, t9542, t9588, t9854, t9865, t9868);
        let (t22223, t22229, t22237, t22240) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1722(t22190, t22203, t22210, t22220, t225, t1877, t73, t4010, t6836, t1353, t5591, t5651);
        let t22252 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1723(t1412, t6816, t1353, t1394, t21969, t1392, t1395, t1877, t1879, t22223, t22229, t22237, t22240, t539, t541, t5644, t5650, t5652, t5655, t6832, t6837, t6840);
    (t22211, t22214, t22215, t22216, t22217, t22218, t22219, t22252)
}
