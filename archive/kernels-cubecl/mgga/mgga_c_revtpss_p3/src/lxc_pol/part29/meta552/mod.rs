//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta552 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1890;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1891;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta552<F: Float>(t26265: F, t9671: F, t26230: F, t94403: F, t25904: F, t4078: F, t689: F, t7492: F, t94589: F, t96279: F, t25895: F, t96239: F, t9686: F, t2098: F, t4075: F, t786: F, t9682: F, t2103: F, t47567: F, t1364: F, t26338: F, t26261: F, t40270: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t96423, t96431, t96432, t96437, t96456, t96458) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1890::<F>(t26265, t9671, t26230, t94403, t25904, t4078, t689, t7492, t94589, t96279, t25895, t96239);
        let (t96460, t96463, t96464, t96473, t96486, t96491) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1891::<F>(t26265, t9686, t2098, t4075, t786, t9682, t2103, t47567, t1364, t26338, t26261, t40270);
    (t96423, t96431, t96432, t96437, t96456, t96458, t96460, t96463, t96464, t96473, t96486, t96491)
}
