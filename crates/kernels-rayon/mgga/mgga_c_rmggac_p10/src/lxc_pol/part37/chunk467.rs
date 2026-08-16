//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 467/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk467(t2841: f64, t321: f64, t333: f64, t352: f64, t11599: f64, t26: f64, t551: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11644 = t2841 * t321;
    let t11648 = t2841 * t333;
    let t11654 = t2841 * t352;
    let t11662 = t11599 * t321;
    let t11666 = t11599 * t333;
    let t11670 = t11599 * t352;
    let t11674 = t26 * t551;
    (t11644, t11648, t11654, t11662, t11666, t11670, t11674)
}
