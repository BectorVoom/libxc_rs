//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta572 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1919;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta572<F: Float>(t14914: F, t25270: F, t14919: F, t14904: F, t27261: F, t14900: F, t4462: F, t92951: F, t14788: F, t14682: F, t14804: F, t14793: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t98940, t98943, t98945, t98947, t98949, t98951, t98953, t98955, t98957) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1919::<F>(t14914, t25270, t14919, t14904, t27261, t14900, t4462, t92951, t14788, t14682, t14804, t14793);
    (t98940, t98943, t98945, t98947, t98949, t98951, t98953, t98955, t98957)
}
