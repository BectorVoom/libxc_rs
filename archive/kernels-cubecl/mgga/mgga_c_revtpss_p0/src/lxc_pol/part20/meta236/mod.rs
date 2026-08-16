//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta236 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1037;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta236<F: Float>(t2645: F, t2723: F, t10115: F, t253: F, t10867: F, t251: F, t233: F, t2760: F, t869: F, t689: F, t2777: F, t2789: F) -> (F, F, F, F, F, F, F) {
        let (t10943, t10948, t10952, t10959, t10960, t10961, t10963) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1037::<F>(t2645, t2723, t10115, t253, t10867, t251, t233, t2760, t869, t689, t2777, t2789);
    (t10943, t10948, t10952, t10959, t10960, t10961, t10963)
}
