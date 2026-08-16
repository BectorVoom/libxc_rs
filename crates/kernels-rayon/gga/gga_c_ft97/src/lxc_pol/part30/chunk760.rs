//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 760/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk760(t7553: f64, t766: f64, t2568: f64, t7546: f64, t10052: f64, t1449: f64, t6187: f64, t6154: f64, t6166: f64, t729: f64, t24412: f64, t6175: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33595 = t7553 * t766;
    let t33596 = t2568 * t33595;
    let t33598 = t7546 * t766;
    let t33599 = t10052 * t33598;
    let t33601 = t1449 * t6187;
    let t33602 = t2568 * t33601;
    let t33605 = t729 * t6154 * t6166;
    let t33608 = t24412 * t6175;
    (t33595, t33596, t33598, t33599, t33601, t33602, t33605, t33608)
}
