//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta305 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1290;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta305<F: Float>(t14: F, t588: F, t521: F, t2496: F, t4038: F, t123: F, t1330: F, t2630: F, t2516: F, t676: F, t3869: F, t3926: F, t3930: F) -> (F, F, F, F, F, F, F, F) {
        let (t9856, t9858, t9861, t9863, t9865, t9866, t9868, t9896) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1290::<F>(t14, t588, t521, t2496, t4038, t123, t1330, t2630, t2516, t676, t3869, t3926, t3930);
    (t9856, t9858, t9861, t9863, t9865, t9866, t9868, t9896)
}
