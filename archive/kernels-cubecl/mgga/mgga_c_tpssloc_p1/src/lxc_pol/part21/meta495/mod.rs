//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta495 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2107;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2108;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2109;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta495<F: Float>(t16662: F, t820: F, t847: F, t2697: F, t5624: F, t13360: F, t1516: F, t5568: F, t9573: F, t2563: F, t5572: F, t16805: F, t237: F, t5576: F, t838: F, t119: F, t210: F, t4180: F, t4181: F, t4234: F, t16839: F, t829: F, t16891: F, t10014: F, t10026: F, t10029: F, t10036: F, t13359: F, t13362: F, t13368: F, t249: F, t2623: F, t2643: F, t5628: F, t787: F, t843: F, t16869: F, t16910: F, t16979: F, t235: F, t5631: F, t814: F, t252: F, t5611: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16985, t16988, t16990, t16993, t16995, t16997) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2107::<F>(t16662, t820, t847, t2697, t5624, t13360, t1516, t5568, t9573, t2563, t5572, t16805, t237);
        let (t17003, t17004, t17009, t17013, t17017, t17020) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2108::<F>(t5576, t838, t119, t16662, t210, t4180, t4181, t4234, t16839, t829, t16891, t10014, t10026, t10029, t10036, t13359, t13362, t13368, t16985, t16988, t16990, t16993, t16995, t16997, t249, t2623, t2643, t5624, t5628, t787, t843);
        let (t17022, t17023, t17027, t17028, t17030) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2109::<F>(t16869, t16910, t16979, t17020, t235, t5631, t814, t829, t252, t5611);
    (t16985, t16997, t17003, t17004, t17009, t17013, t17017, t17022, t17023, t17027, t17028, t17030)
}
