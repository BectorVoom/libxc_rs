//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 306/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk306<F: Float>(t1013: F, t2057: F, t554: F, t1722: F, t1733: F, t2066: F, t3083: F, t3086: F, t3090: F, t3093: F, t3097: F) -> (F, F) {
    let t3393 = t2057 * t1013;
    let t3394 = t3393 * t554;
    let t3404 = -F::cast_from(0.44452000728395061731e-1_f64) * t1722 - t2066 + F::cast_from(0.55565000910493827163e-2_f64) * t1733 - F::cast_from(0.44452000728395061731e-1_f64) * t3083 + F::cast_from(0.55565000910493827163e-2_f64) * t3086 + F::cast_from(0.22226000364197530865e-1_f64) * t3090 - F::cast_from(0.33339000546296296298e-1_f64) * t3093 + F::cast_from(0.33339000546296296298e-1_f64) * t3097;
    (t3394, t3404)
}
