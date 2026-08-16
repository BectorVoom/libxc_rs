//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 956/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk956(t18587: f64, t258: f64, t18217: f64, t18221: f64, t18233: f64, t18387: f64, t18392: f64, t18398: f64, t18492: f64, t18627: f64, t18659: f64, t18750: f64) -> f64 {
    let t18760 = t18587 * t258;
    let t18772 = 2.0_f64 * t18760 - 2.0_f64 * t18392 - 4.0_f64 * t18233 + 8.0_f64 * t18659 - 4.0_f64 * t18221 + 4.0_f64 * t18627 - 12.0_f64 * t18217 + 8.0_f64 * t18750 - 2.0_f64 * t18398 + 4.0_f64 * t18492 - 2.0_f64 * t18387;
    t18772
}
