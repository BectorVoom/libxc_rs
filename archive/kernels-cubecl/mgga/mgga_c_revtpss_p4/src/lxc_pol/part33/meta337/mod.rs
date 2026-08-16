//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta337 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1348;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta337<F: Float>(t12627: F, t487: F, t1269: F, t3566: F, t1203: F, t3565: F, t12295: F, t1204: F, t3766: F, t3555: F, t3754: F, t1248: F, t3153: F) -> (F, F, F, F, F, F, F, F) {
        let (t12628, t12633, t12640, t12641, t12678, t12702, t12709, t12712) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1348::<F>(t12627, t487, t1269, t3566, t1203, t3565, t12295, t1204, t3766, t3555, t3754, t1248, t3153);
    (t12628, t12633, t12640, t12641, t12678, t12702, t12709, t12712)
}
