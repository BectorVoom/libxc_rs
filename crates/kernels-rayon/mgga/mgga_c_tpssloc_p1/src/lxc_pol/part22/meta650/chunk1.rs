//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2191/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2191(t19731: f64, t562: f64, t16576: f64, t751: f64, t2517: f64, t5520: f64, t17109: f64, t870: f64, t16689: f64, t2430: f64, t12945: f64, t4205: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57704 = t562 * t19731;
    let t57887 = t16576 * t751;
    let t57897 = t5520 * t2517;
    let t57932 = t17109 * t870;
    let t57947 = t16689 * t2430;
    let t57960 = t4205 * t12945;
    (t57704, t57887, t57897, t57932, t57947, t57960)
}
