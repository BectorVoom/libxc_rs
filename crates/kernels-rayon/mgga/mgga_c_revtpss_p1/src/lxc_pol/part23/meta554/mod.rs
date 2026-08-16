//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta554 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2111;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2112;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta554(t22369: f64, t2782: f64, t22005: f64, t4086: f64, t543: f64, t6888: f64, t72: f64, t1432: f64, t686: f64, t10049: f64, t10117: f64, t10126: f64, t10129: f64, t10137: f64, t10143: f64, t1399: f64, t14252: f64, t1437: f64, t22009: f64, t22253: f64, t22353: f64, t22362: f64, t22366: f64, t5659: f64, t5735: f64, t5755: f64, t6862: f64, t820: f64, t21998: f64, t22325: f64, t22344: f64, t1427: f64, t213: f64, t13727: f64, t13733: f64, t13737: f64, t1424: f64, t1445: f64, t4071: f64, t5715: f64, t5775: f64, t6896: f64, t9632: f64, t9639: f64, t9642: f64, t9650: f64, t9666: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22370, t22373, t22374, t22379, t22381, t22384) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2111(t22369, t2782, t22005, t4086, t543, t6888, t72, t1432, t686, t10049, t10117, t10126, t10129, t10137, t10143, t1399, t14252, t1437, t22009, t22253, t22353, t22362, t22366, t5659, t5735, t5755, t6862, t820);
        let (t22386, t22387, t22390, t22393) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2112(t21998, t22325, t22344, t22384, t1427, t213, t6888, t13727, t13733, t13737, t1424, t1445, t4071, t5715, t5775, t6896, t9632, t9639, t9642, t9650, t9666);
    (t22370, t22373, t22374, t22379, t22381, t22386, t22387, t22390, t22393)
}
