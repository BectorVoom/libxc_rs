//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta319 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1608;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta319<F: Float>(t2516: F, t5571: F, t5566: F, t72: F, t757: F, t1320: F, t5567: F, t5569: F, t9395: F, t2626: F, t1856: F, t2608: F) -> (F, F, F, F, F, F, F, F) {
        let (t13611, t13613, t13615, t13620, t13621, t13623, t13630, t13632) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1608::<F>(t2516, t5571, t5566, t72, t757, t1320, t5567, t5569, t9395, t2626, t1856, t2608);
    (t13611, t13613, t13615, t13620, t13621, t13623, t13630, t13632)
}
