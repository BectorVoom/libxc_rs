//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta386 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1463;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1464;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1465;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1466;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1467;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta386<F: Float>(t16944: F, t2701: F, t820: F, t5544: F, t776: F, t2697: F, t5628: F, t210: F, t5567: F, t1495: F, t4119: F, t5571: F, t13223: F, t5591: F, t13222: F, t16673: F, t842: F, t13345: F, t13365: F, t1516: F, t16914: F, t16918: F, t16924: F, t16928: F, t16932: F, t16937: F, t16940: F, t16942: F, t2571: F, t2643: F, t4172: F, t4178: F, t4261: F, t5593: F, t843: F, t849: F, t9559: F, t9642: F, t16662: F, t847: F, t5624: F, t13360: F, t5568: F, t9573: F, t2563: F, t5572: F, t16805: F, t237: F, t5576: F, t838: F, t119: F, t4180: F, t4181: F, t4234: F, t16839: F, t829: F, t16891: F, t10014: F, t10026: F, t10029: F, t10036: F, t13359: F, t13362: F, t13368: F, t249: F, t2623: F, t787: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16946, t16949) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1463::<F>(t16944, t2701, t820, t5544, t776);
        let (t16951, t16954, t16957, t16961, t16965, t16968) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1464::<F>(t16949, t2701, t820, t2697, t5628, t210, t5567, t776, t1495, t4119, t5571, t13223, t5591);
        let (t16969, t16979) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1465::<F>(t13222, t16968, t16673, t842, t13345, t13365, t1516, t16914, t16918, t16924, t16928, t16932, t16937, t16940, t16942, t16946, t16951, t16954, t16957, t16961, t16965, t2571, t2643, t4172, t4178, t4261, t5593, t843, t849, t9559, t9642);
        let (t16985, t16988, t16990, t16993, t16995, t16997) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1466::<F>(t16662, t820, t847, t2697, t5624, t13360, t1516, t5568, t9573, t2563, t5572, t16805, t237);
        let (t17004, t17009, t17013, t17017, t17020) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1467::<F>(t5576, t838, t119, t16662, t210, t4180, t4181, t4234, t16839, t829, t16891, t10014, t10026, t10029, t10036, t13359, t13362, t13368, t16985, t16988, t16990, t16993, t16995, t16997, t249, t2623, t2643, t5624, t5628, t787, t843);
    (t16946, t16949, t16951, t16968, t16969, t16979, t16985, t17004, t17009, t17013, t17017, t17020)
}
