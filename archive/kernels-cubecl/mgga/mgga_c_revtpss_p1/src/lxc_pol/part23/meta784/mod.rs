//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta784 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2593;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2594;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta784<F: Float>(t10368: F, t56: F, t1518: F, t670: F, t1921: F, t5789: F, t1913: F, t5808: F, t22532: F, t575: F, t21661: F, t602: F, t2246: F, t5812: F, t1469: F, t627: F, t72: F, t10605: F, t18539: F, t11064: F, t6075: F, t37: F, t5940: F, t2609: F, t5825: F, t706: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t60311, t60595, t60620, t60624, t60629, t60670) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2593::<F>(t10368, t56, t1518, t670, t1921, t5789, t1913, t5808, t22532, t575, t21661, t602);
        let (t60673, t60823, t61020, t61033, t61037, t61090) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2594::<F>(t2246, t5812, t1469, t627, t72, t10605, t18539, t11064, t6075, t37, t5940, t2609, t5825, t706);
    (t60311, t60595, t60620, t60624, t60629, t60670, t60673, t60823, t61020, t61033, t61037, t61090)
}
