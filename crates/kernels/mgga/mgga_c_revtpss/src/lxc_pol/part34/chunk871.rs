//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 871/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk871<F: Float>(t22925: F, t22926: F, t9415: F, t9421: F, t9427: F, t9514: F, t9517: F, t9521: F, t9546: F, t9569: F, t9574: F, t9577: F, t22213: F, t13666: F, t13668: F, t13670: F) -> (F, F, F, F, F) {
    let t22927 = -t9415 + t9421 - t9427 + t9546 + t9514 - t9517 - t9521 + t9569 - t9574 - t9577 - t22925 - t22926;
    let t22928 = 0.17544670867903938621e1 * t22213;
    let t22929 = 0.32530743900905219526e-1 * t13666;
    let t22930 = 36.0 * t13668;
    let t22931 = 96.0 * t13670;
    (t22927, t22928, t22929, t22930, t22931)
}
