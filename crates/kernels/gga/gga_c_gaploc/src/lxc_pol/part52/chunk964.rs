//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 964/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk964<F: Float>(t12256: F, t1991: F, t43363: F, t44090: F, t45170: F, t45174: F, t45176: F, t45177: F, t45178: F, t45179: F, t45180: F, t45183: F, t45187: F, t45188: F, t45192: F, t45193: F, t45194: F, t45195: F, t47140: F, t47145: F, t49989: F, t590: F, t739: F) -> F {
    let t49998 = t45170 - t45174 - F::cast_from(0.38342925953920749676e1_f64) * t43363 - t45176 - t45177 - t45178 + t45179 - t45180 - t45183 + F::cast_from(0.20449560508757733161e1_f64) * t1991 * t739 * t49989 * t590 - F::cast_from(0.14300195980740170668e1_f64) * t12256 * t44090 + t45187 + t45188 + t45192 - t45193 - t45194 + t45195 + F::cast_from(0.10224780254378866581e1_f64) * t47140 + F::cast_from(0.76685851907841499354e0_f64) * t47145;
    t49998
}
