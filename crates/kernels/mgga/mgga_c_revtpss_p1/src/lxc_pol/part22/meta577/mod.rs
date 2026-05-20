//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta577 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2433;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta577<F: Float>(t14328: F, t14334: F, t14336: F, t14339: F, t5819: F, t750: F, t2611: F, t2398: F, t5999: F, t5825: F, t706: F, t4305: F, t4311: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t18535, t18536, t18537, t18538, t18539, t18541, t18543, t18544, t18546, t18547) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2433::<F>(t14328, t14334, t14336, t14339, t5819, t750, t2611, t2398, t5999, t5825, t706, t4305, t4311);
    (t18535, t18536, t18537, t18538, t18539, t18541, t18543, t18544, t18546, t18547)
}
