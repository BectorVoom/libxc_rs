//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 564/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk564<F: Float>(t24: F, t7241: F, t486: F, t100: F, t1570: F, t487: F, t8189: F, t8326: F, t104: F, t7943: F, t89: F, t1786: F, t488: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8411 = t24 * t7241;
    let t8416 = t486 * t486;
    let t8417 = F::cast_from(1.0_f64) / t8416;
    let t8418 = t100 * t8417;
    let t8424 = t487 * t1570;
    let t8455 = F::cast_from(28.0_f64) / F::cast_from(81.0_f64) * t8189;
    let t8518 = t8326 * t100;
    let t8534 = F::cast_from(28.0_f64) / F::cast_from(81.0_f64) * t89 * t7943 * t104;
    let t8557 = t1786 * t488;
    (t8411, t8416, t8417, t8418, t8424, t8455, t8518, t8534, t8557)
}
