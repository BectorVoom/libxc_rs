//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta789 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2603;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2604;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta789<F: Float>(t11044: F, t18797: F, t18317: F, t2435: F, t10871: F, t5977: F, t14931: F, t18477: F, t51123: F, t10811: F, t18471: F, t18451: F, t14923: F, t18634: F, t10726: F, t18408: F, t2661: F, t4366: F, t18608: F, t2662: F, t837: F, t18632: F, t4352: F, t10815: F, t6019: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t61441, t61448, t61532, t61538, t61540, t61542) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2603::<F>(t11044, t18797, t18317, t2435, t10871, t5977, t14931, t18477, t51123, t10811, t18471, t18451);
        let (t61550, t61560, t61564, t61568, t61570) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2604::<F>(t14923, t18634, t10726, t18408, t2661, t4366, t18608, t2662, t837, t18632, t4352, t10815, t6019);
    (t61441, t61448, t61532, t61538, t61540, t61542, t61550, t61560, t61564, t61568, t61570)
}
