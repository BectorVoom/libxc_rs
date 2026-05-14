//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 744/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk744<F: Float>(t730: F, t9434: F, t2552: F, t722: F, t164: F, t172: F, t2555: F, t177: F, t9367: F, t9368: F, t9371: F, t701: F, t9275: F, t2582: F) -> (F, F, F, F, F, F) {
    let t9525 = t9434 * t730;
    let t9529 = 1.0 / t2552 / t722;
    let t9530 = t164 * t9529;
    let t9532 = 1.0 / t2555 / t172;
    let t9533 = t9434 * t9532;
    let t9536 = t177 * t9367;
    let t9537 = t9368 * t9371;
    let t9540 = t9275 * t701;
    let t9542 = 6.0 * t2582 * t9540;
    (t9525, t9530, t9533, t9536, t9537, t9542)
}
