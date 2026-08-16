//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta328 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1331;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta328<F: Float>(t124: F, t836: F, t10779: F, t2749: F, t10777: F, t820: F, t823: F, t844: F, t2751: F, t2681: F, t839: F, t222: F, t9727: F) -> (F, F, F, F, F, F, F) {
        let (t10782, t10783, t10811, t10812, t10815, t10816, t10824) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1331::<F>(t124, t836, t10779, t2749, t10777, t820, t823, t844, t2751, t2681, t839, t222, t9727);
    (t10782, t10783, t10811, t10812, t10815, t10816, t10824)
}
