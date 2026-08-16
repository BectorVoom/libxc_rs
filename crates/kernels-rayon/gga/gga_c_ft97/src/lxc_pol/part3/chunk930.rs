//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 930/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk930(t18252: f64, t18262: f64, t18369: f64, t18384: f64, t762: f64, t242: f64, t5132: f64, t761: f64, t766: f64, t18188: f64, t18190: f64, t18193: f64, t18198: f64, t18203: f64, t18208: f64, t18213: f64, t18218: f64, t18222: f64, t18226: f64, t18230: f64, t18234: f64, t18238: f64, t446: f64) -> (f64, f64, f64) {
    let t18386 = t18252 + t18262 + t18369 + t18384;
    let t18387 = t762 * t18386;
    let t18388 = t242 * t18387;
    let t18391 = t5132 * t761;
    let t18392 = t18391 * t766;
    let t18393 = t242 * t18392;
    let t18396 = t18188 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t18190 + 2.0_f64 / 3.0_f64 * t446 * t18193 + 2.0_f64 / 3.0_f64 * t446 * t18198 + 2.0_f64 / 3.0_f64 * t446 * t18203 - 2.0_f64 * t446 * t18208 - 2.0_f64 / 3.0_f64 * t446 * t18213 - 2.0_f64 * t446 * t18218 - 2.0_f64 / 3.0_f64 * t446 * t18222 - 2.0_f64 / 3.0_f64 * t446 * t18226 - 2.0_f64 / 3.0_f64 * t446 * t18230 - 2.0_f64 / 3.0_f64 * t446 * t18234 - t446 * t18238 / 3.0_f64 - t446 * t18388 / 3.0_f64 - t446 * t18393 / 3.0_f64;
    (t18387, t18392, t18396)
}
