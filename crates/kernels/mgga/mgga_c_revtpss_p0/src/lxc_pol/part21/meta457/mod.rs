//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta457 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1991;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1992;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta457<F: Float>(t2777: F, t4518: F, t2439: F, t2470: F, t4499: F, t2798: F, t1568: F, t2783: F, t786: F, t2801: F, t10533: F, t10539: F, t10543: F, t10548: F, t10645: F, t10647: F, t10651: F, t10655: F, t14546: F, t14547: F, t2646: F, t2724: F, t2754: F, t4494: F, t4504: F, t4514: F, t4526: F, t820: F, t233: F, t4469: F, t869: F, t689: F, t2435: F, t4519: F, t1558: F, t2723: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t14557, t14563, t14567, t14568, t14572) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1991::<F>(t2777, t4518, t2439, t2470, t4499, t2798, t1568, t2783, t786, t2801, t10533, t10539, t10543, t10548, t10645, t10647, t10651, t10655, t14546, t14547, t2646, t2724, t2754, t4494, t4504, t4514, t4526, t820);
        let (t14574, t14575, t14577, t14581, t14586) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1992::<F>(t233, t4469, t869, t689, t2435, t4519, t1558, t2723);
    (t14557, t14563, t14567, t14568, t14572, t14574, t14575, t14577, t14581, t14586)
}
