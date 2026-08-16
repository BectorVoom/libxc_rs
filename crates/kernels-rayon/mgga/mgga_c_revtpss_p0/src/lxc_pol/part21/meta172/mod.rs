//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta172 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1081;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1082;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1083;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1084;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta172(t3829: f64, t4012: f64, t828: f64, t1386: f64, t2482: f64, t27: f64, t136: f64, t1389: f64, t1399: f64, t221: f64, t1317: f64, t1331: f64, t1333: f64, t2522: f64, t2562: f64, t2569: f64, t2579: f64, t2587: f64, t3852: f64, t3854: f64, t3871: f64, t3873: f64, t1330: f64, t749: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4014, t4018) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1081(t3829, t4012, t828, t1386, t2482, t27);
        let t4019 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1082(t136, t1389);
        let (t4021, t4022, t4024, t4025, t4027, t4028) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1083(t1399, t221, t4019, t4018, t1317, t1331, t1333, t2522, t2562, t2569, t2579, t2587, t3852, t3854, t3871, t3873);
        let t4029 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1084(t1330, t749);
    (t4014, t4018, t4019, t4021, t4022, t4024, t4025, t4027, t4028, t4029)
}
