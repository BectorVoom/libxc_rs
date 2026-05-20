//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta362 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1387;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta362<F: Float>(t1178: F, t3519: F, t439: F, t3522: F, t447: F, t300: F, t3488: F, t3800: F, t498: F, t1204: F, t1269: F, t12295: F) -> (F, F, F, F, F, F, F) {
        let (t12552, t12553, t12555, t12571, t12587, t12603, t12610) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1387::<F>(t1178, t3519, t439, t3522, t447, t300, t3488, t3800, t498, t1204, t1269, t12295);
    (t12552, t12553, t12555, t12571, t12587, t12603, t12610)
}
