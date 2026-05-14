//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 733/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk733<F: Float>(t8650: F, t8651: F, t1959: F, t8469: F, t8487: F, t8645: F, t8649: F) -> (F, F) {
    let t8652 = t8650 * t8651;
    let t8656 = 0.56468933516960933999e-3 * t8469 - 0.8673628188205199462e0 * t8645 * t1959 + 0.57119737665102352616e0 * t8649 * t8652 - 0.3718732920905101082e-3 * t8487;
    (t8652, t8656)
}
