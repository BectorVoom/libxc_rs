//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1156/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1156<F: Float>(t265: F, t393: F, t13426: F, t8749: F, t18227: F, t32866: F, t4248: F, t32822: F, t7935: F, t28021: F, t8764: F, t122820: F, t27154: F, t127181: F) -> (F, F, F, F, F, F, F) {
    let t394 = t265 < t393;
    let t129273 = t13426 * t8749;
    let t129277 = t18227 * t8749;
    let t129279 = t4248 * t32866;
    let t129281 = t32822 * t7935;
    let t129283 = t8764 * t28021;
    let t129285 = t122820 * t27154;
    let t129301 = piecewise3::<f64>(t394, F::new(0.0), t127181);
    (t129273, t129277, t129279, t129281, t129283, t129285, t129301)
}
