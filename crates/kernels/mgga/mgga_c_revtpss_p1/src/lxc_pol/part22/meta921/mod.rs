//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta921 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3142;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3143;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta921<F: Float>(t1716: F, t9292: F, t12256: F, t3617: F, t3362: F, t482: F, t12268: F, t1263: F, t12230: F, t5104: F, t3555: F, t488: F, t17807: F, t460: F) -> (F, F, F, F, F, F, F) {
        let t56236 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3142::<F>(t1716, t9292);
        let (t56246, t56250, t56254, t56265, t56294, t56303) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3143::<F>(t12256, t3617, t3362, t482, t12268, t1263, t12230, t5104, t3555, t488, t17807, t460);
    (t56236, t56246, t56250, t56254, t56265, t56294, t56303)
}
