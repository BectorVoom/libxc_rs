//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 947/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk947<F: Float>(t13087: F, t4273: F, t3382: F, t4713: F, t1101: F, t1165: F, t1567: F, t4282: F, t224: F, t4068: F, t1390: F, t709: F, t12930: F, t1549: F, t1554: F, t1558: F) -> (F, F, F, F, F, F, F, F) {
    let t18166 = t13087 * t4273;
    let t18176 = t3382 * t4713;
    let t18189 = t4282 * t1165 * t1567 * t1101;
    let t18217 = t224 * t4068;
    let t18222 = t709 * t1390;
    let t18295 = t12930 * t1549;
    let t18297 = t12930 * t1554;
    let t18299 = t12930 * t1558;
    (t18166, t18176, t18189, t18217, t18222, t18295, t18297, t18299)
}
