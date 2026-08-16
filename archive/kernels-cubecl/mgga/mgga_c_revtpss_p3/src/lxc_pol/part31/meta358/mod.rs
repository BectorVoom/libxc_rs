//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta358 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1379;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1380;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta358<F: Float>(t2398: F, t4305: F, t177: F, t4392: F, t762: F, t2626: F, t4398: F, t10439: F, t162: F, t2516: F, t2496: F, t2619: F, t4302: F, t4186: F, t750: F, t706: F, t4395: F, t4537: F, t892: F, t123: F, t1534: F, t2630: F, t1469: F, t749: F, t606: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14317, t14324, t14328, t14330, t14334, t14336, t14339) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1379::<F>(t2398, t4305, t177, t4392, t762, t2626, t4398, t10439, t162, t2516, t2496, t2619, t4302);
        let (t14343, t14345, t14353, t14363, t14370) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1380::<F>(t4186, t750, t706, t4395, t4537, t892, t123, t1534, t2630, t1469, t749, t606);
    (t14317, t14324, t14328, t14330, t14334, t14336, t14339, t14343, t14345, t14353, t14363, t14370)
}
