//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta449 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1975;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta449<F: Float>(t2619: F, t4302: F, t4186: F, t750: F, t706: F, t4395: F, t10556: F, t4537: F, t892: F, t123: F, t1534: F) -> (F, F, F, F, F, F, F) {
        let (t14340, t14341, t14343, t14345, t14352, t14353, t14362) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1975::<F>(t2619, t4302, t4186, t750, t706, t4395, t10556, t4537, t892, t123, t1534);
    (t14340, t14341, t14343, t14345, t14352, t14353, t14362)
}
