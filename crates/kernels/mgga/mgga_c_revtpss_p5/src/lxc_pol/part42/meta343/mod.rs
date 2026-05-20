//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta343 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1147;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1148;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta343<F: Float>(t1062: F, t11940: F, t3111: F, t4834: F, t11262: F, t1670: F, t1041: F, t3172: F, t4824: F, t3127: F, t3211: F, t4845: F, t1053: F, t4857: F, t1663: F, t371: F, t676: F, t1025: F, t11922: F, t4901: F, t4899: F, t4874: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t15716, t15724, t15732, t15736, t15744) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1147::<F>(t1062, t11940, t3111, t4834, t11262, t1670, t1041, t3172, t4824, t3127, t3211, t4845);
        let (t15745, t15750, t15754, t15771) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1148::<F>(t1053, t4857, t1663, t371, t676, t1025, t11922, t4901, t4899, t3172, t4874, t3127);
    (t15716, t15724, t15732, t15736, t15744, t15745, t15750, t15754, t15771)
}
