//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1264/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1264<F: Float>(t3566: F, t8190: F, t5251: F, t8945: F, t26921: F, t8205: F, t17306: F, t2142: F, t12587: F, t8220: F, t116: F, t30004: F) -> (F, F, F, F, F, F) {
    let t105512 = t3566 * t8190;
    let t105530 = t5251 * t8945;
    let t105558 = t8205 * t26921;
    let t105579 = t17306 * t2142;
    let t105669 = t8220 * t12587;
    let t105819 = t116 * t30004;
    (t105512, t105530, t105558, t105579, t105669, t105819)
}
