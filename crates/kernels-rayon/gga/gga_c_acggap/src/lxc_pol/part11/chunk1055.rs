//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1055/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1055(t1181: f64, t2068: f64, t23445: f64, t604: f64, t30613: f64, t30468: f64, t4425: f64, t4685: f64, t7822: f64, t4331: f64, t1470: f64, t30644: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34497 = t2068 * t1181 * t604 * t23445;
    let t34499 = 0.25724410870841842184e-2_f64 * t30613;
    let t34500 = t30468 * t4425;
    let t34501 = 0.34299214494455789578e-2_f64 * t34500;
    let t34502 = t7822 * t4685;
    let t34504 = t7822 * t4331;
    let t34506 = t30644 * t1470;
    (t34497, t34499, t34501, t34502, t34504, t34506)
}
