//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 786/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk786<F: Float>(t4551: F, t492: F, t8418: F, t83: F, t3255: F, t979: F, t1852: F, t1871: F, t4436: F, t499: F, t1882: F, t4591: F) -> (F, F, F, F, F, F) {
    let t16198 = t4551 * t492;
    let t16199 = t8418 * t16198;
    let t16200 = t83 * t16199;
    let t16203 = t979 * t3255;
    let t16204 = t1852 * t16203;
    let t16205 = t83 * t16204;
    let t16210 = t1871 * t499 * t4436;
    let t16213 = t1882 * t4591;
    (t16199, t16200, t16204, t16205, t16210, t16213)
}
