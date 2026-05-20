//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta619 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2374;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2375;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta619<F: Float>(t10832: F, t10845: F, t820: F, t823: F, t9948: F, t839: F, t10639: F, t221: F, t2484: F, t2485: F, t10820: F, t2652: F, t10841: F, t10878: F, t2741: F, t2722: F, t853: F, t10726: F, t10786: F, t2661: F, t10943: F, t2663: F, t2645: F, t2662: F, t2749: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t40357, t40360, t40361, t40365, t40367) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2374::<F>(t10832, t10845, t820, t823, t9948, t839, t10639, t221, t2484, t2485, t10820, t2652);
        let (t40374, t40376, t40378, t40381, t40385, t40390) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2375::<F>(t10841, t10845, t10878, t2741, t2722, t853, t10726, t10786, t2661, t10943, t2663, t2645, t2662, t2749);
    (t40357, t40360, t40361, t40365, t40367, t40374, t40376, t40378, t40381, t40385, t40390)
}
