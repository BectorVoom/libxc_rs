//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1005/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1005(t1980: f64, t35500: f64, t7476: f64, t1988: f64, t8486: f64, t1967: f64, t8838: f64, t4360: f64, t7741: f64, t13287: f64, t31057: f64, t33953: f64, t5122: f64) -> (f64, f64, f64, f64, f64) {
    let t35502 = t1980 * t7476 * t35500;
    let t35503 = 0.7145669686344956162e-3_f64 * t35502;
    let t35513 = t1988 * t8486;
    let t35514 = 0.94344276868812456204e-3_f64 * t35513;
    let t35515 = t1967 * t8838;
    let t35529 = t7741 * t4360;
    let t35549 = t31057 * t13287 * t33953 * t5122;
    (t35503, t35514, t35515, t35529, t35549)
}
