//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 569/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk569(t103: f64, t4545: f64, t108: f64, t4415: f64, t4501: f64, t4552: f64, t4590: f64, t4594: f64, t4621: f64, t88: f64, t948: f64, t984: f64) -> (f64, f64) {
    let t4623 = t4545 * t103;
    let t4628 = -t108 * t4415 - t108 * t4501 - t4621 * t88 - 2.0_f64 * t948 * t984 + 4.0_f64 * t4552 - 2.0_f64 * t4590 - 4.0_f64 * t4594 + 2.0_f64 * t4623;
    (t4623, t4628)
}
