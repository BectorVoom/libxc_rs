//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta495 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2107;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2108;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2109;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta495(t16662: f64, t820: f64, t847: f64, t2697: f64, t5624: f64, t13360: f64, t1516: f64, t5568: f64, t9573: f64, t2563: f64, t5572: f64, t16805: f64, t237: f64, t5576: f64, t838: f64, t119: f64, t210: f64, t4180: f64, t4181: f64, t4234: f64, t16839: f64, t829: f64, t16891: f64, t10014: f64, t10026: f64, t10029: f64, t10036: f64, t13359: f64, t13362: f64, t13368: f64, t249: f64, t2623: f64, t2643: f64, t5628: f64, t787: f64, t843: f64, t16869: f64, t16910: f64, t16979: f64, t235: f64, t5631: f64, t814: f64, t252: f64, t5611: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16985, t16988, t16990, t16993, t16995, t16997) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2107(t16662, t820, t847, t2697, t5624, t13360, t1516, t5568, t9573, t2563, t5572, t16805, t237);
        let (t17003, t17004, t17009, t17013, t17017, t17020) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2108(t5576, t838, t119, t16662, t210, t4180, t4181, t4234, t16839, t829, t16891, t10014, t10026, t10029, t10036, t13359, t13362, t13368, t16985, t16988, t16990, t16993, t16995, t16997, t249, t2623, t2643, t5624, t5628, t787, t843);
        let (t17022, t17023, t17027, t17028, t17030) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2109(t16869, t16910, t16979, t17020, t235, t5631, t814, t829, t252, t5611);
    (t16985, t16997, t17003, t17004, t17009, t17013, t17017, t17022, t17023, t17027, t17028, t17030)
}
