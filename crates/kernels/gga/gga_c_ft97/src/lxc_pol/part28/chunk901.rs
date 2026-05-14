//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 901/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk901<F: Float>(t136138: F, t144846: F, t32067: F, t144666: F, t28: F, t432: F, t89: F, t3103: F, t32355: F, t136116: F, t942: F, t136241: F, t136243: F, t136250: F, t137070: F, t137073: F, t137102: F, t137623: F, t144946: F, t144950: F, t144953: F, t144956: F, t144961: F) -> (F, F, F, F, F) {
    let t144966 = t32067 * t136138 * t144846;
    let t144970 = t89 * t28 * t144666 * t432;
    let t144974 = t89 * t28 * t32355 * t3103;
    let t144978 = t89 * t28 * t136116 * t942;
    let t144981 = -4.0 / 9.0 * t136241 - t144946 / 12.0 + t136243 / 9.0 - 8.0 / 3.0 * t144950 - 2.0 / 3.0 * t144953 + 2.0 / 9.0 * t144956 + t144961 - 2.0 / 3.0 * t136250 + 2.0 * t137070 - 4.0 / 3.0 * t137073 - 8.0 / 3.0 * t144966 + 2.0 * t144970 + 2.0 * t144974 + 2.0 * t144978 - t137623 - t137102 / 12.0;
    (t144966, t144970, t144974, t144978, t144981)
}
