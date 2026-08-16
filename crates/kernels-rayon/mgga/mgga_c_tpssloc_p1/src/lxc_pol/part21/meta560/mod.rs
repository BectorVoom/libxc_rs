//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta560 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2264;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2265;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta560(t15453: f64, t17686: f64, t4582: f64, t17635: f64, t4972: f64, t1090: f64, t6230: f64, t3578: f64, t6219: f64, t4997: f64, t5002: f64, t11784: f64, t248: f64, t5971: f64, t1227: f64, t5019: f64, t4993: f64, t5005: f64, t1202: f64, t6164: f64, t5024: f64, t11692: f64, t11792: f64, t11821: f64, t15671: f64, t15691: f64, t15699: f64, t15740: f64, t3577: f64, t488: f64, t4950: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18954, t18955, t18958, t18959, t18964, t18965, t18968, t18969, t18972, t18975) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2264(t15453, t17686, t4582, t17635, t4972, t1090, t6230, t3578, t6219, t4997, t5002, t11784, t248, t5971);
        let (t18982, t18989) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2265(t1227, t18975, t4997, t5019, t4993, t5005, t1202, t6164, t5024, t11692, t11792, t11821, t15671, t15691, t15699, t15740, t18955, t18959, t18965, t18969, t18972, t3577, t488, t4950);
    (t18954, t18955, t18958, t18959, t18964, t18965, t18968, t18969, t18975, t18982, t18989)
}
