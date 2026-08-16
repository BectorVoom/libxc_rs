//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta266 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1113;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1114;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta266<F: Float>(t3129: F, t3172: F, t3127: F, t3135: F, t1041: F, t1065: F, t3059: F, t906: F, t1042: F, t1024: F, t3105: F, t3151: F, t3153: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t11643, t11644, t11648, t11649, t11651, t11652, t11653, t11656) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1113::<F>(t3129, t3172, t3127, t3135, t1041, t1065, t3059, t906, t1042, t1024, t3105);
        let t11659 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1114::<F>(t3151, t3153);
    (t11643, t11644, t11648, t11649, t11651, t11652, t11653, t11656, t11659)
}
