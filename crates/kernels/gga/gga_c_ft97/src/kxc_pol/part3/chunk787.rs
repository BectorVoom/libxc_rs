//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 787/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk787<F: Float>(t1825: F, t4589: F, t83: F, t11988: F, t16150: F, t11987: F, t16169: F, t3194: F, t3193: F, t11567: F, t11578: F, t11593: F, t11610: F, t11612: F, t11632: F, t11821: F, t11826: F, t16200: F, t16205: F, t16210: F, t16213: F, t1901: F, t446: F, t8233: F) -> (F, F) {
    let t16215 = t1825 * t4589;
    let t16216 = t83 * t16215;
    let t16219 = t11988 * t16150;
    let t16220 = t11987 * t16219;
    let t16223 = t3194 * t16169;
    let t16224 = t3193 * t16223;
    let t16227 = -t11567 + F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t11578 - F::cast_from(2.0_f64) * t446 * t16200 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t16205 - F::cast_from(4.0_f64) / F::cast_from(81.0_f64) * t8233 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t16210 + t16213 / F::cast_from(9.0_f64) + t11610 - t11612 - t11632 - t446 * t16216 / F::cast_from(3.0_f64) + t11821 - t11826 - F::cast_from(10.0_f64) / F::cast_from(81.0_f64) * t1901 * t16220 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t11593 * t16224;
    (t16215, t16227)
}
