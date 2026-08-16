//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta231 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1368;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1369;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1370;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1371;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta231(t4012: f64, t5627: f64, t828: f64, t3826: f64, t187: f64, t5566: f64, t1856: f64, t72: f64, t757: f64, t2522: f64, t2562: f64, t2569: f64, t2579: f64, t2587: f64, t5546: f64, t5548: f64, t5568: f64, t5570: f64, t5573: f64, t4039: f64, t4032: f64, t4024: f64, t3854: f64, t3859: f64, t3862: f64, t3867: f64, t3871: f64, t3873: f64, t4030: f64, t4035: f64, t4037: f64, t4042: f64, t225: f64, t539: f64, t73: f64, t1412: f64, t1868: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5629, t5632, t5634, t5635, t5637, t5638) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1368(t4012, t5627, t828, t3826, t187, t5566, t1856, t72, t757, t2522, t2562, t2569, t2579, t2587, t5546, t5548, t5568, t5570, t5573);
        let (t5639, t5640, t5641, t5642) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1369(t4039, t4032, t4024, t3854, t3859, t3862, t3867, t3871, t3873, t4030, t4035, t4037, t4042);
        let (t5644, t5650) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1370(t225, t5638, t5642, t539, t73);
        let t5651 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1371(t1412, t1868);
    (t5629, t5632, t5634, t5635, t5637, t5639, t5640, t5641, t5644, t5650, t5651)
}
