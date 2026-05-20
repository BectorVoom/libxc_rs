//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta710 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2466;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2467;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta710<F: Float>(t5603: F, t9692: F, t136: F, t2457: F, t5774: F, t9674: F, t10073: F, t13731: F, t3915: F, t5721: F, t9288: F, t2439: F, t3895: F, t5775: F, t14293: F, t9664: F, t14103: F, t9285: F, t13726: F, t9303: F, t13725: F, t1445: F, t14082: F, t3920: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t47863, t47886, t47899, t47904, t47907) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2466::<F>(t5603, t9692, t136, t2457, t5774, t9674, t10073, t13731, t3915, t5721, t9288, t2439, t3895, t5775);
        let (t47920, t47932, t47938, t47942, t47944) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2467::<F>(t14293, t9664, t14103, t9285, t9674, t13726, t9303, t13725, t1445, t2439, t14082, t3920);
    (t47863, t47886, t47899, t47904, t47907, t47920, t47932, t47938, t47942, t47944)
}
