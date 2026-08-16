//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta318 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1325;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta318<F: Float>(t10654: F, t2782: F, t2760: F, t822: F, t243: F, t816: F, t9707: F, t813: F, t2394: F, t2476: F, t236: F, t807: F) -> (F, F, F, F, F, F) {
        let (t10655, t10657, t10671, t10673, t10674, t10676) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1325::<F>(t10654, t2782, t2760, t822, t243, t816, t9707, t813, t2394, t2476, t236, t807);
    (t10655, t10657, t10671, t10673, t10674, t10676)
}
