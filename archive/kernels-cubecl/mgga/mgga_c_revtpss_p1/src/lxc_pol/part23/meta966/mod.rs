//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta966 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3264;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta966<F: Float>(t48262: F, t47011: F, t48269: F, t22789: F, t72: F, t757: F, t73476: F, t39783: F, t39786: F, t39791: F, t39795: F, t39799: F, t39807: F, t39813: F, t47059: F, t48261: F, t48266: F, t48268: F, t48271: F) -> (F, F, F, F, F, F) {
        let (t85908, t85909, t85910, t85913, t85914, t85915) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3264::<F>(t48262, t47011, t48269, t22789, t72, t757, t73476, t39783, t39786, t39791, t39795, t39799, t39807, t39813, t47059, t48261, t48266, t48268, t48271);
    (t85908, t85909, t85910, t85913, t85914, t85915)
}
