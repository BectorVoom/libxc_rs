//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta348 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1652;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1653;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta348<F: Float>(t2777: F, t4518: F, t2439: F, t2470: F, t4499: F, t2798: F, t1568: F, t2783: F, t786: F, t2801: F, t233: F, t4469: F, t869: F, t689: F, t2435: F, t4519: F, t1558: F, t2723: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14557, t14558, t14563, t14564, t14567, t14568) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1652::<F>(t2777, t4518, t2439, t2470, t4499, t2798, t1568, t2783, t786);
        let (t14570, t14574, t14575, t14577, t14581, t14586) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1653::<F>(t14568, t2801, t233, t4469, t869, t689, t2435, t4519, t1558, t2723);
    (t14557, t14558, t14563, t14564, t14567, t14568, t14570, t14574, t14575, t14577, t14581, t14586)
}
