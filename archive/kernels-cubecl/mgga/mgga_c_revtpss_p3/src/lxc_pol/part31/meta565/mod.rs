//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta565 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1976;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta565<F: Float>(t543: F, t74700: F, t116: F, t21813: F, t5966: F, t890: F, t5962: F, t1544: F, t4537: F, t5876: F, t670: F, t1448: F, t6836: F) -> (F, F, F, F, F, F, F) {
        let (t75305, t75439, t77408, t77425, t77441, t85360, t86753) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1976::<F>(t543, t74700, t116, t21813, t5966, t890, t5962, t1544, t4537, t5876, t670, t1448, t6836);
    (t75305, t75439, t77408, t77425, t77441, t85360, t86753)
}
