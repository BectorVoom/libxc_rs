//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 759/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk759<F: Float>(t9365: F, t9368: F, t1110: F, t119: F, t1101: F, t2697: F, t267: F, t918: F, t9364: F) -> (F, F, F, F, F, F) {
    let t9369 = t9365 * t9368;
    let t9371 = t1110 * t119;
    let t9373 = t1101 * t9371 * t2697;
    let t9375 = t267 * t918;
    let t9377 = t1101 * t9375 * t2697;
    let t9379 = t1101 * t9364;
    (t9369, t9371, t9373, t9375, t9377, t9379)
}
