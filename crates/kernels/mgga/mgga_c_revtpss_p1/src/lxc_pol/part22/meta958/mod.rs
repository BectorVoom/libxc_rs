//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta958 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3216;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta958<F: Float>(t49864: F, t10605: F, t18539: F, t49866: F, t39423: F, t39425: F, t39433: F, t39436: F, t14365: F, t18865: F, t2403: F, t39419: F, t39422: F, t39429: F, t39432: F) -> (F, F, F, F, F, F, F, F) {
        let (t61019, t61021, t61022, t61026, t61027, t61028, t61029, t61030) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3216::<F>(t49864, t10605, t18539, t49866, t39423, t39425, t39433, t39436, t14365, t18865, t2403, t39419, t39422, t39429, t39432);
    (t61019, t61021, t61022, t61026, t61027, t61028, t61029, t61030)
}
