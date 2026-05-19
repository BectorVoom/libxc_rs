//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 869/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk869<F: Float>(t2828: F, t886: F, t2770: F, t2435: F, t2445: F, t2441: F, t9303: F, t10115: F, t258: F, t2453: F, t2464: F, t2438: F) -> (F, F, F, F, F, F) {
    let t10494 = t886 * t2828;
    let t10495 = t2770 * t10494;
    let t10498 = t2435 * t2445;
    let t10501 = F::cast_from(0.26019841438354088051e-2_f64) * t9303 * t2441;
    let t10503 = F::cast_from(0.11044544084478153697e-3_f64) * t10115 * t258;
    let t10504 = t2453 * t2464;
    let t10505 = t2438 * t886;
    (t10495, t10498, t10501, t10503, t10504, t10505)
}
