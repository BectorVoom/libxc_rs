//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1173/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1173<F: Float>(t94456: F, t94460: F, t94462: F, t94464: F, t94466: F, t94468: F, t94472: F, t94474: F, t94477: F, t94479: F, t94481: F, t94484: F, t94485: F, t94487: F, t64: F, t9990: F) -> (F, F) {
    let t94489 = -0.12004725073059526352e-1 * t94456 - 0.34013387707001991332e-1 * t94460 - 0.42874018118069736972e-3 * t94462 + 0.25724410870841842184e-1 * t94464 - 0.42874018118069736972e-3 * t94466 - 0.76230004213927992339e-4 * t94468 - t94472 + t94474 - t94477 + 0.60984003371142393869e-4 * t94479 + 3.0 / 16.0 * t94481 + t94484 + 7.0 / 48.0 * t94485 - t94487 / 48.0;
    let t94491 = t9990 * t64;
    (t94489, t94491)
}
