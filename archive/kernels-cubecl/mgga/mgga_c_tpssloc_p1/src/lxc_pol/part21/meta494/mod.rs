//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta494 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2104;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2105;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2106;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta494<F: Float>(t16935: F, t4180: F, t4181: F, t2639: F, t5619: F, t5614: F, t1484: F, t4119: F, t2701: F, t820: F, t5544: F, t776: F, t2697: F, t5628: F, t210: F, t5567: F, t1495: F, t5571: F, t13223: F, t5591: F, t13222: F, t16673: F, t842: F, t13345: F, t13365: F, t1516: F, t16914: F, t16918: F, t16924: F, t16928: F, t16932: F, t2571: F, t2643: F, t4172: F, t4178: F, t4261: F, t5593: F, t843: F, t849: F, t9559: F, t9642: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t16937, t16940, t16942, t16944, t16946, t16949) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2104::<F>(t16935, t4180, t4181, t2639, t5619, t5614, t1484, t4119, t2701, t820, t5544, t776);
        let (t16951, t16954, t16957, t16961, t16965, t16968) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2105::<F>(t16949, t2701, t820, t2697, t5628, t210, t5567, t776, t1495, t4119, t5571, t13223, t5591);
        let (t16969, t16976, t16979) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2106::<F>(t13222, t16968, t16673, t842, t13345, t13365, t1516, t16914, t16918, t16924, t16928, t16932, t16937, t16940, t16942, t16946, t16951, t16954, t16957, t16961, t16965, t2571, t2643, t4172, t4178, t4261, t5593, t843, t849, t9559, t9642);
    (t16937, t16944, t16946, t16949, t16951, t16957, t16961, t16965, t16969, t16976, t16979)
}
