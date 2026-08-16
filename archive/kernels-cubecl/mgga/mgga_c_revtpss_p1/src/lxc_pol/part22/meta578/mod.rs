//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta578 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2434;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta578<F: Float>(t18547: F, t14363: F, t162: F, t18298: F, t187: F, t10563: F, t14324: F, t14343: F, t14345: F, t14372: F, t18535: F, t18536: F, t18537: F, t18538: F, t18541: F, t18543: F, t18546: F, t9394: F) -> (F, F, F, F, F) {
        let (t18548, t18549, t18550, t18552, t18553) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2434::<F>(t18547, t14363, t162, t18298, t187, t10563, t14324, t14343, t14345, t14372, t18535, t18536, t18537, t18538, t18541, t18543, t18546, t9394);
    (t18548, t18549, t18550, t18552, t18553)
}
