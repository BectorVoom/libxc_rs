//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta798 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2622;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2623;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta798<F: Float>(t14370: F, t18259: F, t18562: F, t2626: F, t14330: F, t5819: F, t606: F, t749: F, t162: F, t50089: F, t2609: F, t5944: F, t18263: F, t2615: F, t2475: F, t5962: F, t10696: F, t5966: F, t18616: F, t221: F, t2484: F, t2485: F, t10815: F, t5980: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t62274, t62276, t62282, t62291, t62300) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2622::<F>(t14370, t18259, t18562, t2626, t14330, t5819, t606, t749, t162, t50089, t2609, t5944);
        let (t62302, t62351, t62361, t62392, t62399) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2623::<F>(t18263, t2615, t2475, t5962, t10696, t5966, t18616, t221, t2484, t2485, t10815, t5980);
    (t62274, t62276, t62282, t62291, t62300, t62302, t62351, t62361, t62392, t62399)
}
