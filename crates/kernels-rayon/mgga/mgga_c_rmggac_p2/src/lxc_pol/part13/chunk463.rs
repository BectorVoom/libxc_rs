//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 463/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk463(t107: f64, t622: f64, t1656: f64, t290: f64, t552: f64, t839: f64, t1602: f64, t321: f64, t333: f64, t559: f64, t848: f64, t1587: f64, t338: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5058 = t622 * t107;
    let t5061 = t290 * t1656;
    let t5064 = t552 * t839;
    let t5072 = t1602 * t321;
    let t5076 = t1602 * t333;
    let t5095 = t559 * t848;
    let t5098 = t338 * t1587;
    (t5058, t5061, t5064, t5072, t5076, t5095, t5098)
}
