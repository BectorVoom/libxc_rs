//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1851/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1851<F: Float>(t27440: F, t7160: F, t7810: F, t988: F, t7145: F, t4820: F, t7122: F, t4878: F, t7121: F) -> (F, F, F, F) {
    let t27441 = t7160 * t27440;
    let t27444 = t7810 * t988;
    let t27445 = t7145 * t27444;
    let t27448 = t7122 * t4820;
    let t27450 = t4878 * t7121;
    (t27441, t27445, t27448, t27450)
}
