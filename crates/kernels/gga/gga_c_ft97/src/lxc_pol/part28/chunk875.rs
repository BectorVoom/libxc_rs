//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 875/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk875<F: Float>(t160: F, t32869: F, t1882: F, t33052: F, t33171: F, t33147: F, t33133: F, t1637: F, t7392: F, t89: F, t33024: F, t33142: F, t33020: F, t33151: F, t33121: F, t376: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t140338 = t160 * t32869;
    let t140364 = t1882 * t33052;
    let t140370 = t1882 * t33171;
    let t140376 = t1882 * t33147;
    let t140378 = t1882 * t33133;
    let t140382 = 4.0 / 27.0 * t89 * t1637 * t7392;
    let t140383 = t1882 * t33024;
    let t140390 = t1882 * t33142;
    let t140395 = t1882 * t33020;
    let t140397 = t1882 * t33151;
    let t140412 = t89 * t376 * t33121;
    (t140338, t140364, t140370, t140376, t140378, t140382, t140383, t140390, t140395, t140397, t140412)
}
