//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta344 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1646;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta344<F: Float>(t2382: F, t4186: F, t2615: F, t4311: F, t1469: F, t2609: F, t706: F, t80: F, t83: F, t1568: F, t785: F, t780: F) -> (F, F, F, F, F, F, F, F) {
        let (t14416, t14433, t14440, t14441, t14447, t14458, t14472, t14473) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1646::<F>(t2382, t4186, t2615, t4311, t1469, t2609, t706, t80, t83, t1568, t785, t780);
    (t14416, t14433, t14440, t14441, t14447, t14458, t14472, t14473)
}
