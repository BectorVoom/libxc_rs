//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 734/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk734(t230: f64, t709: f64, t420: f64, t1418: f64, t6051: f64, t7453: f64, t33365: f64, t3766: f64) -> (f64, f64, f64, f64, f64) {
    let t33373 = t230 * t709;
    let t33374 = t420 * t33373;
    let t33375 = t1418 * t33374;
    let t33379 = 0.25537443351851851852e-1_f64 * t7453 * t6051;
    let t33380 = t3766 * t33365;
    (t33373, t33374, t33375, t33379, t33380)
}
