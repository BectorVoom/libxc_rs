//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 953/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk953<F: Float>(t10309: F, t32135: F, t644: F, t8441: F, t8621: F, t38: F, t8437: F, t2247: F, t36: F, t606: F, t1925: F, t8435: F) -> (F, F, F, F, F, F, F) {
    let t32136 = t10309 * t32135;
    let t32137 = t8441 * t644;
    let t32138 = t8621 * t32137;
    let t32141 = t38 * t8437;
    let t32142 = t2247 * t32141;
    let t32143 = t8441 * t36;
    let t32145 = t8621 * t32143 * t606;
    let t32148 = t8435 * t1925;
    (t32136, t32138, t32141, t32142, t32143, t32145, t32148)
}
