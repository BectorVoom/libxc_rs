//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta941 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3091;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3092;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta941<F: Float>(t20645: F, t57818: F, t1149: F, t12227: F, t16668: F, t6470: F, t1189: F, t1196: F, t24407: F, t3495: F, t16676: F, t6535: F, t16784: F, t6548: F, t24494: F, t3531: F, t5181: F, t6555: F, t20896: F, t5192: F, t81352: F, t81558: F, t81560: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t81562, t81566, t81570, t81573) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3091::<F>(t20645, t57818, t1149, t12227, t16668, t6470, t1189, t1196, t24407, t3495, t16676, t6535);
        let (t81575, t81577, t81580, t81582, t81583) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3092::<F>(t16784, t6548, t24494, t3531, t1196, t5181, t6555, t20896, t5192, t81352, t81558, t81560, t81562, t81566, t81570, t81573);
    (t81562, t81566, t81570, t81573, t81575, t81577, t81580, t81582, t81583)
}
