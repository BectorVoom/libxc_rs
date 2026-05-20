//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 978/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk978<F: Float>(t14587: F, t28425: F, t26497: F, t4481: F, t26550: F, t27349: F, t14495: F, t27312: F, t212: F, t7997: F, t780: F, t689: F) -> (F, F, F, F, F, F) {
    let t28426 = t28425 * t14587;
    let t28434 = t26497 * t4481;
    let t28436 = t26550 * t27349;
    let t28439 = t26550 * t14495;
    let t28442 = t26550 * t27312;
    let t28447 = t212 * t7997;
    let t28448 = t28447 * t780;
    let t28449 = t689 * t28448;
    (t28426, t28434, t28436, t28439, t28442, t28449)
}
