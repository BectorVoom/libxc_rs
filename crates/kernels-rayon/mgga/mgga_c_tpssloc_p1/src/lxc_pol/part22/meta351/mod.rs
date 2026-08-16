//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta351 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1562;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1563;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta351(t13222: f64, t16968: f64, t16673: f64, t842: f64, t13345: f64, t13365: f64, t1516: f64, t16914: f64, t16918: f64, t16924: f64, t16928: f64, t16932: f64, t16937: f64, t16940: f64, t16942: f64, t16946: f64, t16951: f64, t16954: f64, t16957: f64, t16961: f64, t16965: f64, t2571: f64, t2643: f64, t4172: f64, t4178: f64, t4261: f64, t5593: f64, t843: f64, t849: f64, t9559: f64, t9642: f64, t16662: f64, t820: f64, t847: f64, t2697: f64, t5624: f64, t13360: f64, t5568: f64, t9573: f64, t2563: f64, t5572: f64, t16805: f64, t237: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16969, t16976, t16979) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1562(t13222, t16968, t16673, t842, t13345, t13365, t1516, t16914, t16918, t16924, t16928, t16932, t16937, t16940, t16942, t16946, t16951, t16954, t16957, t16961, t16965, t2571, t2643, t4172, t4178, t4261, t5593, t843, t849, t9559, t9642);
        let (t16985, t16988, t16990, t16993, t16995, t16997) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1563(t16662, t820, t847, t2697, t5624, t13360, t1516, t5568, t9573, t2563, t5572, t16805, t237);
    (t16969, t16976, t16979, t16985, t16988, t16990, t16993, t16995, t16997)
}
