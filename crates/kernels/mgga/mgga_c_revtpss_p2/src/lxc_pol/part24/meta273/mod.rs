//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta273 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1046;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta273<F: Float>(t125: F, t5966: F, t2652: F, t5993: F, t6030: F, t10858: F, t6024: F, t2741: F, t6019: F, t10811: F, t6037: F, t221: F, t2485: F, t5978: F) -> (F, F, F, F, F, F, F) {
        let (t18469, t18475, t18485, t18487, t18491, t18518, t18531) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1046::<F>(t125, t5966, t2652, t5993, t6030, t10858, t6024, t2741, t6019, t10811, t6037, t221, t2485, t5978);
    (t18469, t18475, t18485, t18487, t18491, t18518, t18531)
}
