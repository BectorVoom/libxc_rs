//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta355 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1762;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1763;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1764;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1765;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta355(t2645: f64, t2684: f64, t4248: f64, t13076: f64, t13080: f64, t13084: f64, t13087: f64, t13173: f64, t13177: f64, t13182: f64, t13186: f64, t13190: f64, t13193: f64, t13198: f64, t13202: f64, t13204: f64, t13208: f64, t2623: f64, t2643: f64, t2681: f64, t4167: f64, t4178: f64, t4257: f64, t787: f64, t817: f64, t831: f64, t843: f64, t9602: f64, t9604: f64, t2644: f64, t820: f64, t1509: f64, t828: f64, t2647: f64, t2632: f64, t776: f64, t1500: f64, t2693: f64, t4163: f64, t838: f64, t120: f64, t4233: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13210, t13213) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1762(t2645, t2684, t4248, t13076, t13080, t13084, t13087, t13173, t13177, t13182, t13186, t13190, t13193, t13198, t13202, t13204, t13208, t2623, t2643, t2681, t4167, t4178, t4257, t787, t817, t831, t843, t9602, t9604);
        let t13222 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1763(t2644, t820);
        let (t13223, t13225, t13228) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1764(t1509, t828, t2647, t13222, t2632);
        let (t13229, t13231, t13234, t13237, t13242) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1765(t776, t828, t13228, t13222, t1500, t2693, t4163, t838, t120, t4233);
    (t13210, t13213, t13222, t13223, t13225, t13228, t13229, t13231, t13234, t13237, t13242)
}
