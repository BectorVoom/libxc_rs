//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta321 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1330;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1331;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta321<F: Float>(t10727: F, t2723: F, t10726: F, t2661: F, t231: F, t2662: F, t221: F, t2430: F, t2675: F, t2674: F, t2735: F, t2783: F, t2664: F, t808: F, t2693: F, t2710: F, t2713: F, t810: F, t9784: F, t9789: F, t235: F, t2453: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10728, t10730, t10732, t10734, t10741, t10742, t10744) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1330::<F>(t10727, t2723, t10726, t2661, t231, t2662, t221, t2430, t2675, t2674, t2735, t2783);
        let (t10746, t10749, t10756, t10758, t10760) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1331::<F>(t2664, t808, t10744, t2693, t2710, t2713, t810, t9784, t9789, t235, t2783, t2453);
    (t10728, t10730, t10732, t10734, t10741, t10742, t10744, t10746, t10749, t10756, t10758, t10760)
}
