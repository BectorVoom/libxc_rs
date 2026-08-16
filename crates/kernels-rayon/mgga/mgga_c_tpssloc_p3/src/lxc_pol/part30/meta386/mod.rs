//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta386 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1463;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1464;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1465;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1466;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1467;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta386(t16944: f64, t2701: f64, t820: f64, t5544: f64, t776: f64, t2697: f64, t5628: f64, t210: f64, t5567: f64, t1495: f64, t4119: f64, t5571: f64, t13223: f64, t5591: f64, t13222: f64, t16673: f64, t842: f64, t13345: f64, t13365: f64, t1516: f64, t16914: f64, t16918: f64, t16924: f64, t16928: f64, t16932: f64, t16937: f64, t16940: f64, t16942: f64, t2571: f64, t2643: f64, t4172: f64, t4178: f64, t4261: f64, t5593: f64, t843: f64, t849: f64, t9559: f64, t9642: f64, t16662: f64, t847: f64, t5624: f64, t13360: f64, t5568: f64, t9573: f64, t2563: f64, t5572: f64, t16805: f64, t237: f64, t5576: f64, t838: f64, t119: f64, t4180: f64, t4181: f64, t4234: f64, t16839: f64, t829: f64, t16891: f64, t10014: f64, t10026: f64, t10029: f64, t10036: f64, t13359: f64, t13362: f64, t13368: f64, t249: f64, t2623: f64, t787: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16946, t16949) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1463(t16944, t2701, t820, t5544, t776);
        let (t16951, t16954, t16957, t16961, t16965, t16968) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1464(t16949, t2701, t820, t2697, t5628, t210, t5567, t776, t1495, t4119, t5571, t13223, t5591);
        let (t16969, t16979) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1465(t13222, t16968, t16673, t842, t13345, t13365, t1516, t16914, t16918, t16924, t16928, t16932, t16937, t16940, t16942, t16946, t16951, t16954, t16957, t16961, t16965, t2571, t2643, t4172, t4178, t4261, t5593, t843, t849, t9559, t9642);
        let (t16985, t16988, t16990, t16993, t16995, t16997) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1466(t16662, t820, t847, t2697, t5624, t13360, t1516, t5568, t9573, t2563, t5572, t16805, t237);
        let (t17004, t17009, t17013, t17017, t17020) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1467(t5576, t838, t119, t16662, t210, t4180, t4181, t4234, t16839, t829, t16891, t10014, t10026, t10029, t10036, t13359, t13362, t13368, t16985, t16988, t16990, t16993, t16995, t16997, t249, t2623, t2643, t5624, t5628, t787, t843);
    (t16946, t16949, t16951, t16968, t16969, t16979, t16985, t17004, t17009, t17013, t17017, t17020)
}
