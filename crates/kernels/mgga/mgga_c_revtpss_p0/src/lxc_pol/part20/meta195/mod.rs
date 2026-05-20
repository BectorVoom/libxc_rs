//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta195 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk956;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk957;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk958;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta195<F: Float>(t4021: F, t9976: F, t1398: F, t1412: F, t3938: F, t3992: F, t2661: F, t1353: F, t3889: F, t4012: F, t828: F, t1384: F, t235: F, t239: F, t820: F, t4003: F, t543: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t9977, t9979, t9981, t9982, t9984, t9986, t9989) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk956::<F>(t4021, t9976, t1398, t1412, t3938, t3992, t2661, t1353, t3889, t4012, t828, t1384);
        let (t9990, t9991) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk957::<F>(t9989, t235);
        let (t9993, t9994) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk958::<F>(t239, t820, t9991, t4003, t543);
    (t9977, t9979, t9981, t9982, t9984, t9986, t9989, t9990, t9991, t9993, t9994)
}
