//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1380/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1380<F: Float>(t1882: F, t27295: F, t26880: F, t26885: F, t6710: F, t8232: F, t26846: F, t27012: F, t8392: F, t104527: F, t12968: F, t144: F, t1901: F, t2142: F, t23463: F, t26836: F, t3450: F, t446: F, t574: F, t95919: F, t95936: F, t95938: F, t95948: F, t95954: F, t95956: F) -> (F,) {
    let t107156 = 2.0 / 9.0 * t1882 * t27295;
    let t107168 = 2.0 / 9.0 * t1882 * t26880;
    let t107170 = 2.0 / 9.0 * t1882 * t26885;
    let t107177 = t8232 * t6710;
    let t107180 = 2.0 / 9.0 * t1882 * t26846;
    let t107183 = 4.0 / 9.0 * t8392 * t27012;
    let t107184 = t107156 + t95919 / 9.0 + t95936 / 9.0 + t95938 / 9.0 - t446 * t144 * t104527 / 3.0 - 4.0 / 3.0 * t1901 * t12968 * t23463 * t3450 - t107168 - t107170 + 2.0 / 3.0 * t446 * t574 * t2142 * t26836 + 2.0 / 3.0 * t95948 + 8.0 / 81.0 * t95954 + 8.0 / 27.0 * t107177 - t107180 + t95956 / 9.0 + t107183;
    (t107184,)
}
