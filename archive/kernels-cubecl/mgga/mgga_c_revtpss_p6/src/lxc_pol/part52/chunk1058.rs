//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1058/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1058<F: Float>(t7003: F, t7359: F, t7316: F, t8698: F, t2007: F, t7373: F, t196: F, t197: F, t7484: F, t2035: F, t7313: F, t531: F, t8713: F) -> (F, F, F, F, F, F, F) {
    let t32619 = F::cast_from(2.0_f64) * t7359 * t7003;
    let t32620 = t8698 * t7316;
    let t32621 = t2007 * t7373;
    let t32626 = t7484 * t196 * t197;
    let t32627 = t32626 * t2035;
    let t32628 = t8698 * t7313;
    let t32629 = t531 * t8713;
    (t32619, t32620, t32621, t32626, t32627, t32628, t32629)
}
