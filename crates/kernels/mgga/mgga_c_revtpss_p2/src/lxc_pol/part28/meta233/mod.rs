//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta233 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1079;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta233<F: Float>(t1250: F, t482: F, t5284: F, t1042: F, t1038: F, t1802: F, t1244: F, t1241: F, t1121: F, t1263: F) -> (F, F, F, F, F) {
        let (t5286, t5287, t5292, t5293, t5296) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1079::<F>(t1250, t482, t5284, t1042, t1038, t1802, t1244, t1241, t1121, t1263);
    (t5286, t5287, t5292, t5293, t5296)
}
