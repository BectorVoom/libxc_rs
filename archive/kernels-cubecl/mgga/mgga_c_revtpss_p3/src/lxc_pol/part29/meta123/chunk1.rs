//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 671/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk671<F: Float>(t225: F, t2735: F, t826: F, t849: F, t820: F, t823: F, t843: F) -> (F, F, F, F) {
    let t2736 = t2735 * t225;
    let t2737 = t849 * t826;
    let t2739 = F::cast_from(0.25410001404642664112e-5_f64) * t2736 * t2737;
    let t2741 = t820 * t823 * t843;
    (t2736, t2737, t2739, t2741)
}
