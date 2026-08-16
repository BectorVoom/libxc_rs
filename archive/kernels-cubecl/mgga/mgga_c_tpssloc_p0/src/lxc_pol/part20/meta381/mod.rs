//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta381 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1738;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1739;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1740;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1741;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta381<F: Float>(t13191: F, t2701: F, t820: F, t1484: F, t2553: F, t2563: F, t4159: F, t119: F, t12971: F, t210: F, t4155: F, t9573: F, t2645: F, t2684: F, t4248: F, t13076: F, t13080: F, t13084: F, t13087: F, t13173: F, t13177: F, t13182: F, t13186: F, t13190: F, t2623: F, t2643: F, t2681: F, t4167: F, t4178: F, t4257: F, t787: F, t817: F, t831: F, t843: F, t9602: F, t9604: F, t2644: F, t1509: F, t828: F, t2647: F, t2632: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13193, t13196, t13198, t13202, t13203, t13204, t13208) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1738::<F>(t13191, t2701, t820, t1484, t2553, t2563, t4159, t119, t12971, t210, t4155, t9573);
        let (t13210, t13213) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1739::<F>(t2645, t2684, t4248, t13076, t13080, t13084, t13087, t13173, t13177, t13182, t13186, t13190, t13193, t13198, t13202, t13204, t13208, t2623, t2643, t2681, t4167, t4178, t4257, t787, t817, t831, t843, t9602, t9604);
        let t13222 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1740::<F>(t2644, t820);
        let (t13223, t13225, t13228) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1741::<F>(t1509, t828, t2647, t13222, t2632);
    (t13193, t13196, t13198, t13203, t13204, t13210, t13213, t13222, t13223, t13225, t13228)
}
