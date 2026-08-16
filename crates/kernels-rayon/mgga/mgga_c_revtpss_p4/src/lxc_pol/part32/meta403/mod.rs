//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta403 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1390;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1391;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1392;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta403(t18615: f64, t231: f64, t827: f64, t828: f64, t221: f64, t2485: f64, t6017: f64, t2484: f64, t125: f64, t5962: f64, t2747: f64, t837: f64, t2723: f64, t4423: f64, t4364: f64, t4365: f64, t4343: f64, t10779: f64, t14671: f64, t6035: f64, t10777: f64, t14676: f64, t18444: f64, t14894: f64, t14907: f64, t14925: f64, t14934: f64, t18527: f64, t18532: f64, t2745: f64, t4362: f64, t825: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18616, t18618, t18622, t18623, t18629) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1390(t18615, t231, t827, t828, t221, t2485, t6017, t2484, t125, t5962, t2747, t837);
        let (t18632, t18634, t18639, t18643, t18644, t18647) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1391(t2723, t4423, t4364, t4365, t231, t4343, t2747, t10779, t14671, t6035, t10777, t14676);
        let (t18651, t18654) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1392(t18444, t4364, t837, t14894, t14907, t14925, t14934, t18527, t18532, t18618, t18623, t18629, t18634, t18639, t18644, t18647, t2745, t4362, t825);
    (t18616, t18618, t18622, t18629, t18632, t18634, t18639, t18643, t18647, t18651, t18654)
}
