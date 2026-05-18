//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1195/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1195<F: Float>(t13287: F, t2302: F, t34823: F, t8791: F, t1761: F, t30644: F, t5807: F, t7822: F, t6153: F, t6157: F, t7647: F, t1713: F, t31491: F, t7381: F) -> (F, F, F, F, F, F) {
    let t40465 = t34823 * t13287 * t2302 * t8791;
    let t40467 = t30644 * t1761;
    let t40469 = t7822 * t5807;
    let t40472 = t7822 * t6153;
    let t40474 = t7647 * t6157;
    let t40477 = t31491 * t7381 * t1713;
    (t40465, t40467, t40469, t40472, t40474, t40477)
}
