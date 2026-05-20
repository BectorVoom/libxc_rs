//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta852 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2993;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2994;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta852<F: Float>(t4186: F, t4401: F, t606: F, t749: F, t14362: F, t9575: F, t123: F, t2630: F, t4392: F, t4398: F, t9318: F, t15071: F, t892: F, t14322: F, t2516: F, t2496: F, t14426: F, t177: F, t762: F, t10428: F, t4305: F, t2609: F, t706: F, t10436: F, t4311: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t49911, t49926, t49929, t49940, t49950) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2993::<F>(t4186, t4401, t606, t749, t14362, t9575, t123, t2630, t4392, t4398, t9318, t15071, t892);
        let (t49957, t49963, t49966, t49978, t49981, t49983) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2994::<F>(t14322, t2516, t2496, t14426, t177, t762, t10428, t4305, t2609, t4186, t706, t10436, t4311);
    (t49911, t49926, t49929, t49940, t49950, t49957, t49963, t49966, t49978, t49981, t49983)
}
