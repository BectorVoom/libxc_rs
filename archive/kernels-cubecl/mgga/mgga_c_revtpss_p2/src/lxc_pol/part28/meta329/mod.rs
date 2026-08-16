//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta329 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1343;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta329<F: Float>(t10994: F, t786: F, t2771: F, t676: F, t123: F, t2435: F, t2448: F, t2440: F, t887: F, t2439: F, t866: F, t225: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t10995, t10996, t10997, t10998, t11000, t11004, t11006, t11007, t11008) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1343::<F>(t10994, t786, t2771, t676, t123, t2435, t2448, t2440, t887, t2439, t866, t225);
    (t10995, t10996, t10997, t10998, t11000, t11004, t11006, t11007, t11008)
}
