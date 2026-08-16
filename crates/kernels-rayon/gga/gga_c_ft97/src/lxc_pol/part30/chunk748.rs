//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 748/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk748(t33460: f64, t505: f64, t9770: f64, t446: f64, t33243: f64, t713: f64, t193: f64, t89: f64, t6008: f64, t6061: f64, t375: f64, t7532: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33462 = t9770 * t33460 * t505;
    let t33463 = t446 * t33462;
    let t33465 = t33243 * t713;
    let t33466 = t193 * t33465;
    let t33467 = t89 * t33466;
    let t33469 = t6008 * t6061;
    let t33470 = t193 * t33469;
    let t33471 = t89 * t33470;
    let t33474 = t89 * t375 * t7532;
    (t33462, t33463, t33465, t33467, t33469, t33471, t33474)
}
