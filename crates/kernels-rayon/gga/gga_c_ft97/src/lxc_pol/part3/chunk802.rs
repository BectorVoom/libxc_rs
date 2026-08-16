//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 802/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk802(t16417: f64, t16461: f64, t457: f64, t91: f64, t1766: f64, t4533: f64, t473: f64, t3119: f64, t3157: f64, t4505: f64, t8345: f64, t11043: f64, t11076: f64, t11404: f64, t11946: f64, t11957: f64, t8260: f64, t8451: f64) -> (f64, f64, f64, f64, f64) {
    let t16462 = t16417 + t16461;
    let t16464 = t91 * t457 * t16462;
    let t16467 = t1766 * t4533;
    let t16469 = t91 * t16467 * t473;
    let t16472 = t91 * t3119 * t3157;
    let t16474 = t8345 * t4505;
    let t16476 = t91 * t16474 * t473;
    let t16478 = -t8451 - 8.0_f64 / 27.0_f64 * t11043 + t11946 - 8.0_f64 / 9.0_f64 * t11076 - t8260 + t16464 / 2.0_f64 + 4.0_f64 / 9.0_f64 * t11404 - t11957 - t16469 / 4.0_f64 - t16472 / 2.0_f64 + 3.0_f64 / 8.0_f64 * t16476;
    (t16464, t16469, t16472, t16476, t16478)
}
