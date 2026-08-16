//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1250/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1250(t322: f64, t41940: f64, t41971: f64, t42003: f64, t42035: f64, t42067: f64, t42098: f64, t42131: f64, t12029: f64, t37271: f64, t12094: f64, t37282: f64, t12215: f64, t40549: f64) -> (f64, f64, f64, f64) {
    let t323 = t322 <= 0.0_f64;
    let t331 = t322 <= 0.25e1_f64;
    let t42133 = piecewise5(t323, t41940, t331, t41971 + t42003 + t42035 + t42067, t42098 + t42131);
    let t42136 = 5.0_f64 / 8.0_f64 * t37271 * t12029;
    let t42138 = 15.0_f64 / 8.0_f64 * t37282 * t12094;
    let t42140 = 3.0_f64 * t40549 * t12215;
    (t42133, t42136, t42138, t42140)
}
