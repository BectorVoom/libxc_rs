//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta371 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1406;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1407;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta371(t10726: f64, t14868: f64, t2661: f64, t10868: f64, t241: f64, t820: f64, t10811: f64, t4452: f64, t2719: f64, t844: f64, t4368: f64, t2482: f64, t814: f64, t14671: f64, t14686: f64, t4366: f64, t136: f64, t1568: f64, t2457: f64, t2710: f64, t2470: f64, t4522: f64, t874: f64, t4469: f64, t822: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14871, t14894, t14907, t14925, t14931) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1406(t10726, t14868, t2661, t10868, t241, t820, t10811, t4452, t2719, t844, t4368, t2482, t814);
        let (t14933, t14934, t14948, t14951, t14972) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1407(t14671, t14686, t4366, t14931, t136, t1568, t2457, t2710, t2470, t4522, t874, t4469, t822);
    (t14871, t14894, t14907, t14925, t14933, t14934, t14948, t14951, t14972)
}
