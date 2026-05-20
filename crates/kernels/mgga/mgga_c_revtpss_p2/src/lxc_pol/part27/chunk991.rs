//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 991/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk991<F: Float>(t11642: F, t11701: F, t11751: F, t11799: F, t11850: F, t11919: F, t11976: F, t12029: F, t225: F, t385: F, t3270: F, t999: F) -> (F, F, F) {
    let t12032 = t11642 + t11701 + t11751 + t11799 + t11850 + t11919 + t11976 + t12029;
    let t12034 = t12032 * t225 * t385;
    let t12039 = t999 * t3270;
    (t12032, t12034, t12039)
}
