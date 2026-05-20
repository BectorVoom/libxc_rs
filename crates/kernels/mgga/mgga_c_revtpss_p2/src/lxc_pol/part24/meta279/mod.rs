//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta279 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1053;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta279<F: Float>(t6071: F, t72: F, t686: F, t2465: F, t213: F, t6041: F, t6048: F, t10995: F, t6072: F, t779: F, t689: F, t1580: F, t4321: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t18796, t18797, t18798, t18800, t18804, t18805, t18806, t18811, t18812, t18814) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1053::<F>(t6071, t72, t686, t2465, t213, t6041, t6048, t10995, t6072, t779, t689, t1580, t4321);
    (t18796, t18797, t18798, t18800, t18804, t18805, t18806, t18811, t18812, t18814)
}
