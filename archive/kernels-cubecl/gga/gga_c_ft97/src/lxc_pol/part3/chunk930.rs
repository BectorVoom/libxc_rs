//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 930/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk930<F: Float>(t18252: F, t18262: F, t18369: F, t18384: F, t762: F, t242: F, t5132: F, t761: F, t766: F, t18188: F, t18190: F, t18193: F, t18198: F, t18203: F, t18208: F, t18213: F, t18218: F, t18222: F, t18226: F, t18230: F, t18234: F, t18238: F, t446: F) -> (F, F, F) {
    let t18386 = t18252 + t18262 + t18369 + t18384;
    let t18387 = t762 * t18386;
    let t18388 = t242 * t18387;
    let t18391 = t5132 * t761;
    let t18392 = t18391 * t766;
    let t18393 = t242 * t18392;
    let t18396 = t18188 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t18190 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t18193 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t18198 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t18203 - F::cast_from(2.0_f64) * t446 * t18208 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t18213 - F::cast_from(2.0_f64) * t446 * t18218 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t18222 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t18226 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t18230 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t18234 - t446 * t18238 / F::cast_from(3.0_f64) - t446 * t18388 / F::cast_from(3.0_f64) - t446 * t18393 / F::cast_from(3.0_f64);
    (t18387, t18392, t18396)
}
