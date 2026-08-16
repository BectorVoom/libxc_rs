//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta820 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2933;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2934;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta820<F: Float>(t4132: F, t5599: F, t689: F, t14103: F, t9285: F, t9674: F, t13730: F, t1420: F, t2782: F, t13726: F, t9303: F, t13725: F, t1445: F, t2439: F, t14082: F, t3920: F, t14078: F, t2470: F, t3915: F, t13735: F, t2435: F, t10119: F, t14114: F, t10115: F, t1900: F, t14189: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t47929, t47932, t47936, t47938, t47942) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2933::<F>(t4132, t5599, t689, t14103, t9285, t9674, t13730, t1420, t2782, t13726, t9303, t13725, t1445, t2439);
        let (t47944, t47947, t47952, t47957, t47961, t47963) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2934::<F>(t14082, t3920, t14078, t2470, t3915, t13735, t2435, t10119, t14114, t10115, t1900, t14189);
    (t47929, t47932, t47936, t47938, t47942, t47944, t47947, t47952, t47957, t47961, t47963)
}
