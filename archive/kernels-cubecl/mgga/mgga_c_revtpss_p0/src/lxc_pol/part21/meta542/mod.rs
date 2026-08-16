//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta542 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2205;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta542<F: Float>(t1150: F, t16942: F, t1131: F, t1168: F, t5143: F, t1745: F, t3471: F, t12423: F, t16649: F, t16651: F, t16654: F, t16657: F, t16660: F, t16664: F, t16667: F, t16671: F, t16690: F, t3452: F, t5147: F) -> (F, F, F, F, F) {
        let (t16943, t16945, t16948, t16951, t16954) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2205::<F>(t1150, t16942, t1131, t1168, t5143, t1745, t3471, t12423, t16649, t16651, t16654, t16657, t16660, t16664, t16667, t16671, t16690, t3452, t5147);
    (t16943, t16945, t16948, t16951, t16954)
}
