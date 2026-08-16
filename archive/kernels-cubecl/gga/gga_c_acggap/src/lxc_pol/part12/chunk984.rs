//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 984/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk984<F: Float>(t315: F, t323: F, t8301: F, t29979: F, t29980: F, t638: F, t15758: F, t32041: F, t8306: F, t2176: F, t3883: F, t32142: F, t8085: F) -> (F, F, F, F, F) {
    let t33147 = t315 * t8301 * t323;
    let t33150 = t29979 * t638 * t29980;
    let t33153 = t32041 * t8306 * t15758;
    let t33155 = t2176 * t3883;
    let t33157 = t32142 * t8085;
    (t33147, t33150, t33153, t33155, t33157)
}
