//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 909/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk909<F: Float>(t10988: F, t689: F, t2444: F, t887: F, t252: F, t2769: F, t786: F, t2771: F, t676: F, t123: F, t2435: F, t2448: F) -> (F, F, F, F, F, F) {
    let t10989 = t689 * t10988;
    let t10991 = t2444 * t887;
    let t10992 = t689 * t10991;
    let t10994 = t252 * t2769;
    let t10995 = t786 * t10994;
    let t10996 = t676 * t2771;
    let t10997 = t123 * t10996;
    let t10998 = t10995 * t10997;
    let t11000 = t2435 * t2448;
    (t10989, t10992, t10996, t10997, t10998, t11000)
}
