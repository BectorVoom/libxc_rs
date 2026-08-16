//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta398 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1376;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1377;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1378;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta398(t18426: f64, t4364: f64, t4366: f64, t2741: f64, t5980: f64, t4365: f64, t4424: f64, t837: f64, t125: f64, t5966: f64, t10770: f64, t2652: f64, t5993: f64, t14586: f64, t14786: f64, t14791: f64, t1559: f64, t4433: f64, t14785: f64, t6030: f64, t10858: f64, t6024: f64, t10816: f64, t10824: f64, t10826: f64, t2745: f64, t4362: f64, t6019: f64, t775: f64, t10698: f64, t828: f64, t1544: f64, t4343: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18456, t18459, t18462, t18466, t18471, t18475) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1376(t18426, t4364, t4366, t2741, t5980, t4365, t4424, t837, t125, t5966, t10770, t2652, t5993);
        let (t18478, t18482, t18489) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1377(t14586, t14786, t14791, t1559, t4433, t14785, t2652, t6030, t10858, t6024, t10816, t10824, t10826, t18456, t18459, t18462, t18466, t18471, t18475, t2745, t4362);
        let (t18491, t18493, t18495, t18498) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1378(t2741, t6019, t5966, t775, t10698, t828, t1544, t4343);
    (t18456, t18462, t18466, t18471, t18478, t18482, t18489, t18491, t18493, t18495, t18498)
}
