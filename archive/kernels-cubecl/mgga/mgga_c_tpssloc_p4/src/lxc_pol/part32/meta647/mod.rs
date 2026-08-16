//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta647 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2069;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2070;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta647<F: Float>(t90864: F, t26433: F, t6883: F, t22716: F, t7741: F, t22704: F, t5336: F, t80798: F, t22724: F, t26436: F, t26423: F, t81159: F, t215: F, t22839: F, t562: F, t80854: F, t1338: F, t26328: F, t26462: F, t6914: F, t22705: F, t26414: F, t81228: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t90865, t90867, t90868, t90899, t90900, t90912) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2069::<F>(t90864, t26433, t6883, t22716, t7741, t22704, t5336, t80798, t22724, t26436, t26423, t81159);
        let (t90913, t90914, t90915, t90952, t90957, t90961) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2070::<F>(t90912, t215, t22839, t562, t80854, t1338, t26328, t26462, t6914, t22705, t26414, t81228);
    (t90865, t90867, t90868, t90899, t90900, t90913, t90914, t90915, t90952, t90957, t90961)
}
