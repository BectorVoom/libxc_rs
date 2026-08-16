//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1056/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1056(t34506: f64, t30984: f64, t8458: f64, t2268: f64, t30456: f64, t1562: f64, t30948: f64, t1444: f64, t1992: f64, t30154: f64, t7586: f64, t30596: f64, t30607: f64, t30611: f64, t34482: f64, t34484: f64, t34485: f64, t34489: f64, t34492: f64, t34497: f64, t34499: f64, t34501: f64, t34502: f64, t34504: f64) -> f64 {
    let t34507 = 0.17149607247227894789e-2_f64 * t34506;
    let t34508 = t30984 * t8458;
    let t34510 = t30456 * t2268;
    let t34512 = t30948 * t1562;
    let t34513 = 0.16006300097412701803e-1_f64 * t34512;
    let t34516 = t30154 * t7586 * t1992 * t1444;
    let t34518 = -0.25724410870841842183e-2_f64 * t34482 + t30596 - t34484 - t34485 + 0.140078125e-1_f64 * t30607 + t34489 - 0.15724046144802076034e-3_f64 * t34492 - 0.25724410870841842184e-2_f64 * t30611 + 0.62896184579208304136e-3_f64 * t34497 - t34499 + t34501 - 0.17149607247227894789e-2_f64 * t34502 - 0.85748036236139473944e-3_f64 * t34504 + t34507 - 0.15724046144802076034e-2_f64 * t34508 + 0.66040993808168719343e-2_f64 * t34510 - t34513 + 0.20965394859736101379e-2_f64 * t34516;
    t34518
}
