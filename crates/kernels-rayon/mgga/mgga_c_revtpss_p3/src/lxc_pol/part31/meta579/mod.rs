//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta579 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1997;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1998;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta579(t25310: f64, t25331: f64, t2435: f64, t25339: f64, t11064: f64, t7086: f64, t25604: f64, t995: f64, t357: f64, t988: f64, t355: f64, t1071: f64, t11239: f64, t1078: f64, t1982: f64, t25610: f64, t3093: f64, t4975: f64, t3058: f64, t8521: f64, t3143: f64, t7135: f64, t11865: f64, t25516: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t93384, t93391, t93404, t93436, t93438, t93488) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1997(t25310, t25331, t2435, t25339, t11064, t7086, t25604, t995, t357, t988, t355, t1071, t11239);
        let (t93490, t93497, t93498, t93502, t93516, t93543) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1998(t1078, t1982, t93488, t25604, t25610, t3093, t4975, t3058, t8521, t3143, t7135, t11865, t25516);
    (t93384, t93391, t93404, t93436, t93438, t93490, t93497, t93498, t93502, t93516, t93543)
}
