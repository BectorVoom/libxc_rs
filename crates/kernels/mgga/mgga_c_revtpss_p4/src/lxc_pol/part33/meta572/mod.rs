//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta572 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1981;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta572<F: Float>(t10115: F, t555: F, t4146: F, t198: F, t775: F, t13026: F, t65: F, t2246: F, t4171: F, t10308: F, t1466: F, t21661: F, t602: F) -> (F, F, F, F, F, F, F) {
        let (t47567, t47672, t50080, t57549, t60221, t60224, t60670) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1981::<F>(t10115, t555, t4146, t198, t775, t13026, t65, t2246, t4171, t10308, t1466, t21661, t602);
    (t47567, t47672, t50080, t57549, t60221, t60224, t60670)
}
