//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta355 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1762;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1763;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1764;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1765;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta355<F: Float>(t2645: F, t2684: F, t4248: F, t13076: F, t13080: F, t13084: F, t13087: F, t13173: F, t13177: F, t13182: F, t13186: F, t13190: F, t13193: F, t13198: F, t13202: F, t13204: F, t13208: F, t2623: F, t2643: F, t2681: F, t4167: F, t4178: F, t4257: F, t787: F, t817: F, t831: F, t843: F, t9602: F, t9604: F, t2644: F, t820: F, t1509: F, t828: F, t2647: F, t2632: F, t776: F, t1500: F, t2693: F, t4163: F, t838: F, t120: F, t4233: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13210, t13213) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1762::<F>(t2645, t2684, t4248, t13076, t13080, t13084, t13087, t13173, t13177, t13182, t13186, t13190, t13193, t13198, t13202, t13204, t13208, t2623, t2643, t2681, t4167, t4178, t4257, t787, t817, t831, t843, t9602, t9604);
        let t13222 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1763::<F>(t2644, t820);
        let (t13223, t13225, t13228) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1764::<F>(t1509, t828, t2647, t13222, t2632);
        let (t13229, t13231, t13234, t13237, t13242) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1765::<F>(t776, t828, t13228, t13222, t1500, t2693, t4163, t838, t120, t4233);
    (t13210, t13213, t13222, t13223, t13225, t13228, t13229, t13231, t13234, t13237, t13242)
}
