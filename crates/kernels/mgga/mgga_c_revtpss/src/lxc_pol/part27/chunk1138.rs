//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1138/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1138<F: Float>(t28167: F, t49616: F, t8717: F, t1450: F, t2014: F, t2033: F, t9400: F, t10259: F, t572: F, t7330: F, t117: F, t94991: F, t116: F, t25832: F, t670: F, t2371: F, t26123: F) -> (F, F, F, F, F, F) {
    let t95104 = 18.0 * t28167 * t8717 * t49616;
    let t95108 = 6.0 * t2014 * t9400 * t2033 * t1450;
    let t95131 = 6.0 * t572 * t7330 * t10259;
    let t95136 = 3.0 * t572 * t117 * t94991;
    let t95137 = t116 * t25832;
    let t95140 = 18.0 * t572 * t95137 * t670;
    let t95143 = 18.0 * t572 * t26123 * t2371;
    (t95104, t95108, t95131, t95136, t95140, t95143)
}
