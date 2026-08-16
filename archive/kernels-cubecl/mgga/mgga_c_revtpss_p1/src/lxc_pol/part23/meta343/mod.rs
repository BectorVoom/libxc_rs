//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta343 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1645;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta343<F: Float>(t14362: F, t2630: F, t1469: F, t749: F, t606: F, t4401: F, t4391: F, t705: F, t10446: F, t2375: F, t4186: F, t10457: F) -> (F, F, F, F, F, F, F, F) {
        let (t14363, t14369, t14370, t14372, t14386, t14401, t14404, t14413) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1645::<F>(t14362, t2630, t1469, t749, t606, t4401, t4391, t705, t10446, t2375, t4186, t10457);
    (t14363, t14369, t14370, t14372, t14386, t14401, t14404, t14413)
}
