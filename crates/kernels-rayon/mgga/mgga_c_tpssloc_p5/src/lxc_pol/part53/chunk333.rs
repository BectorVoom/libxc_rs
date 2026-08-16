//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 333/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk333(t1615: f64, t360: f64, t1021: f64, t248: f64, t1044: f64, t1539: f64, t1020: f64, t1038: f64, t1041: f64, t1607: f64, t1612: f64, t378: f64, t973: f64, t997: f64) -> (f64, f64, f64, f64) {
    let t1616 = t1615 * t360;
    let t1618 = t248 * t1021 * t1616;
    let t1622 = t248 * t1044 * t1539;
    let t1625 = t997 + t973 * t1607 / 288.0_f64 + t1612 * t378 / 3072.0_f64 + t1020 * t1618 / 3072.0_f64 + t1038 + t1041 * t1622 / 4608.0_f64;
    (t1616, t1618, t1622, t1625)
}
