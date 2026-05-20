//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta184 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk790;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta184<F: Float>(t1330: F, t749: F, t512: F, t1320: F, t1331: F, t1340: F, t2516: F, t2496: F, t177: F, t762: F, t2626: F, t3827: F, t3856: F, t3859: F, t3862: F, t3865: F, t3867: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4029, t4030, t4031, t4032, t4033, t4035, t4037, t4038, t4039, t4040, t4042, t4043) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk790::<F>(t1330, t749, t512, t1320, t1331, t1340, t2516, t2496, t177, t762, t2626, t3827, t3856, t3859, t3862, t3865, t3867);
    (t4029, t4030, t4031, t4032, t4033, t4035, t4037, t4038, t4039, t4040, t4042, t4043)
}
