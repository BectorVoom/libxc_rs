//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 834/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk834<F: Float>(t1882: F, t20184: F, t20244: F, t20236: F, t8392: F, t20417: F, t20397: F, t20395: F, t487: F, t20172: F, t20265: F, t20210: F, t20435: F, t20401: F, t20226: F, t20439: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t74863 = t1882 * t20184;
    let t74865 = t1882 * t20244;
    let t74883 = t8392 * t20236;
    let t74899 = t1882 * t20417;
    let t74902 = t1882 * t20397;
    let t74959 = t487 * t20395;
    let t74992 = t8392 * t20172;
    let t75034 = t1882 * t20265;
    let t75048 = t8392 * t20210;
    let t75050 = t8392 * t20435;
    let t75071 = t1882 * t20401;
    let t75115 = t8392 * t20226;
    let t75117 = t8392 * t20439;
    (t74863, t74865, t74883, t74899, t74902, t74959, t74992, t75034, t75048, t75050, t75071, t75115, t75117)
}
