//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta294 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1077;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta294<F: Float>(t1770: F, t1811: F, t3172: F, t6618: F, t3711: F, t6634: F, t3610: F, t5265: F, t5293: F, t3153: F, t6628: F) -> (F, F, F, F, F, F, F) {
        let (t20756, t20783, t20784, t20786, t20787, t20789, t20795) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1077::<F>(t1770, t1811, t3172, t6618, t3711, t6634, t3610, t5265, t5293, t3153, t6628);
    (t20756, t20783, t20784, t20786, t20787, t20789, t20795)
}
