//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 904/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk904(t112760: f64, t2649: f64, t30714: f64, t23109: f64, t23110: f64, t232: f64, t59: f64, t828: f64, t1894: f64, t23078: f64, t2379: f64, t23062: f64, t30700: f64) -> (f64, f64, f64, f64, f64) {
    let t112761 = 0.76763589786250567036e-1_f64 * t112760;
    let t112773 = t30714 * t2649;
    let t112778 = t23109 * t23110 * t59 * t828 * t232;
    let t112782 = t23078 * t1894 * t59 * t2379;
    let t112784 = t23062 * t30700;
    (t112761, t112773, t112778, t112782, t112784)
}
