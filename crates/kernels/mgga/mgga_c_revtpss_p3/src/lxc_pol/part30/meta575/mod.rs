//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta575 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2025;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta575<F: Float>(t94385: F, t94386: F, t94383: F, t25304: F, t555: F, t25898: F, t25876: F, t25931: F, t25894: F, t1444: F, t543: F, t268: F, t4102: F) -> (F, F, F, F, F, F, F) {
        let (t94388, t94390, t94391, t94392, t94394, t94395, t94398) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2025::<F>(t94385, t94386, t94383, t25304, t555, t25898, t25876, t25931, t25894, t1444, t543, t268, t4102);
    (t94388, t94390, t94391, t94392, t94394, t94395, t94398)
}
