//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 963/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk963<F: Float>(t27213: F, t7407: F, t2061: F, t2718: F, t26497: F, t4481: F, t212: F, t7997: F, t780: F, t689: F, t2411: F, t8019: F) -> (F, F, F, F, F, F, F) {
    let t28422 = t27213 * t7407;
    let t28425 = t2718 * t2061;
    let t28434 = t26497 * t4481;
    let t28447 = t212 * t7997;
    let t28448 = t28447 * t780;
    let t28449 = t689 * t28448;
    let t28460 = t8019 * t2411;
    (t28422, t28425, t28434, t28447, t28448, t28449, t28460)
}
