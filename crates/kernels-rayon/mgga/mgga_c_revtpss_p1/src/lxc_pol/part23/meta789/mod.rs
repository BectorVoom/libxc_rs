//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta789 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2603;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2604;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta789(t11044: f64, t18797: f64, t18317: f64, t2435: f64, t10871: f64, t5977: f64, t14931: f64, t18477: f64, t51123: f64, t10811: f64, t18471: f64, t18451: f64, t14923: f64, t18634: f64, t10726: f64, t18408: f64, t2661: f64, t4366: f64, t18608: f64, t2662: f64, t837: f64, t18632: f64, t4352: f64, t10815: f64, t6019: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t61441, t61448, t61532, t61538, t61540, t61542) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2603(t11044, t18797, t18317, t2435, t10871, t5977, t14931, t18477, t51123, t10811, t18471, t18451);
        let (t61550, t61560, t61564, t61568, t61570) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2604(t14923, t18634, t10726, t18408, t2661, t4366, t18608, t2662, t837, t18632, t4352, t10815, t6019);
    (t61441, t61448, t61532, t61538, t61540, t61542, t61550, t61560, t61564, t61568, t61570)
}
