//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1186/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1186(t2030: f64, t35413: f64, t5697: f64, t34903: f64, t5693: f64, t7450: f64, t372: f64, t4262: f64, t9529: f64, t1298: f64, t2297: f64, t4256: f64) -> (f64, f64, f64, f64) {
    let t40347 = t2030 * t35413 * t5697;
    let t40350 = t7450 * t34903 * t5693;
    let t40354 = t7450 * t4262 * t9529 * t372;
    let t40358 = t7450 * t4256 * t2297 * t1298;
    (t40347, t40350, t40354, t40358)
}
