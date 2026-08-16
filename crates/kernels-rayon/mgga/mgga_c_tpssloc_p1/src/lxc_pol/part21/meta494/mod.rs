//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta494 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2104;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2105;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2106;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta494(t16935: f64, t4180: f64, t4181: f64, t2639: f64, t5619: f64, t5614: f64, t1484: f64, t4119: f64, t2701: f64, t820: f64, t5544: f64, t776: f64, t2697: f64, t5628: f64, t210: f64, t5567: f64, t1495: f64, t5571: f64, t13223: f64, t5591: f64, t13222: f64, t16673: f64, t842: f64, t13345: f64, t13365: f64, t1516: f64, t16914: f64, t16918: f64, t16924: f64, t16928: f64, t16932: f64, t2571: f64, t2643: f64, t4172: f64, t4178: f64, t4261: f64, t5593: f64, t843: f64, t849: f64, t9559: f64, t9642: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16937, t16940, t16942, t16944, t16946, t16949) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2104(t16935, t4180, t4181, t2639, t5619, t5614, t1484, t4119, t2701, t820, t5544, t776);
        let (t16951, t16954, t16957, t16961, t16965, t16968) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2105(t16949, t2701, t820, t2697, t5628, t210, t5567, t776, t1495, t4119, t5571, t13223, t5591);
        let (t16969, t16976, t16979) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2106(t13222, t16968, t16673, t842, t13345, t13365, t1516, t16914, t16918, t16924, t16928, t16932, t16937, t16940, t16942, t16946, t16951, t16954, t16957, t16961, t16965, t2571, t2643, t4172, t4178, t4261, t5593, t843, t849, t9559, t9642);
    (t16937, t16944, t16946, t16949, t16951, t16957, t16961, t16965, t16969, t16976, t16979)
}
