//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta151 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk825;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk826;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk827;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta151(t1399: f64, t221: f64, t4019: f64, t4018: f64, t1317: f64, t1331: f64, t1333: f64, t2522: f64, t2562: f64, t2569: f64, t2579: f64, t2587: f64, t3852: f64, t3854: f64, t3871: f64, t3873: f64, t1330: f64, t749: f64, t512: f64, t1320: f64, t1340: f64, t2516: f64, t2496: f64, t177: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4021, t4022, t4025, t4027, t4028) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk825(t1399, t221, t4019, t4018, t1317, t1331, t1333, t2522, t2562, t2569, t2579, t2587, t3852, t3854, t3871, t3873);
        let t4029 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk826(t1330, t749);
        let (t4031, t4033, t4035, t4037, t4038) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk827(t4029, t512, t1320, t1331, t1340, t2516, t2496, t1330, t177);
    (t4021, t4022, t4025, t4027, t4028, t4029, t4031, t4033, t4035, t4037, t4038)
}
