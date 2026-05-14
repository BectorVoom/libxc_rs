//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 741/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk741<F: Float>(t167: F, t32907: F, t9432: F, t609: F, t7312: F, t2185: F, t605: F, t32912: F, t1882: F, t7354: F, t144: F, t32735: F, t1391: F, t574: F, t5842: F, t7402: F) -> (F, F, F, F, F, F, F, F) {
    let t33133 = t9432 * t167 * t32907;
    let t33136 = t7312 * t609;
    let t33138 = t2185 * t605 * t33136;
    let t33142 = t2185 * t167 * t32912;
    let t33146 = 2.0 / 9.0 * t1882 * t7354;
    let t33147 = t144 * t32735;
    let t33151 = t574 * t1391 * t5842;
    let t33155 = 2.0 / 9.0 * t1882 * t7402;
    (t33133, t33136, t33138, t33142, t33146, t33147, t33151, t33155)
}
