//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1392/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1392<F: Float>(t13633: F, t13615: F, t13620: F, t13623: F, t13634: F, t13635: F, t22192: F, t22194: F, t22196: F, t22197: F, t22198: F, t22199: F, t22200: F, t22201: F, t9394: F, t9415: F) -> (F, F) {
    let t22202 = F::new(2.0) * t13633;
    let t22203 = -t22192 + t22194 + t22196 - t22197 - t13615 + t9394 - t13620 - t22198 - t13623 - t22199 - t22200 + t22201 + t22202 + t13634 - t13635 - t9415;
    (t22202, t22203)
}
