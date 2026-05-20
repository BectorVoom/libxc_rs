//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta369 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1204;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1205;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta369<F: Float>(t14328: F, t14334: F, t14336: F, t14339: F, t5819: F, t750: F, t2611: F, t2398: F, t5999: F, t5825: F, t706: F, t4305: F, t4311: F, t14363: F, t162: F, t18298: F, t187: F, t10563: F, t14324: F, t14343: F, t14345: F, t14372: F, t9394: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t18535, t18536, t18537, t18538, t18541, t18543, t18546, t18547) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1204::<F>(t14328, t14334, t14336, t14339, t5819, t750, t2611, t2398, t5999, t5825, t706, t4305, t4311);
        let (t18548, t18549, t18552, t18553) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1205::<F>(t18547, t14363, t162, t18298, t187, t10563, t14324, t14343, t14345, t14372, t18535, t18536, t18537, t18538, t18541, t18543, t18546, t9394);
    (t18535, t18536, t18537, t18538, t18541, t18543, t18546, t18548, t18549, t18552, t18553)
}
