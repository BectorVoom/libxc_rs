//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 749/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk749(t6086: f64, t6087: f64, t6085: f64, t489: f64, t537: f64, t146: f64, t252: f64) -> (f64, f64, f64) {
    let t6088 = t6086 * t6087;
    let t6089 = t6085 * t6088;
    let t6091 = t489 * t537;
    let t6093 = t146 * t6091 * t252;
    (t6089, t6091, t6093)
}
