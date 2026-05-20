//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta378 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1415;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta378<F: Float>(t1062: F, t11940: F, t3111: F, t4834: F, t11262: F, t1670: F, t1041: F, t3172: F, t4824: F, t3127: F, t3211: F, t4845: F) -> (F, F, F, F, F, F, F) {
        let (t15716, t15724, t15731, t15732, t15734, t15736, t15744) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1415::<F>(t1062, t11940, t3111, t4834, t11262, t1670, t1041, t3172, t4824, t3127, t3211, t4845);
    (t15716, t15724, t15731, t15732, t15734, t15736, t15744)
}
