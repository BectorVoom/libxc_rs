//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1155/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1155(t42112: f64, t2859: f64, t2884: f64, t302: f64, t41654: f64, t41961: f64, t2887: f64, t271: f64, t2770: f64, t41666: f64, t10468: f64, t191: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t42113 = 1.0_f64 / t42112;
    let t42154 = t302 / t2884 / t2859;
    let t42212 = 0.5356037037037037037e1_f64 * t41654;
    let t42213 = 0.16979925925925925926e1_f64 * t41961;
    let t42224 = t2884 * t2884;
    let t42226 = t302 / t42224;
    let t42227 = t2887 * t2887;
    let t42228 = 1.0_f64 / t42227;
    let t42245 = 0.17757530864197530864e0_f64 * t41654;
    let t42308 = 1.0_f64 / t271 / t2770;
    let t42309 = t42308 * t41666;
    let t42339 = 1.0_f64 / t10468 / t191;
    (t42113, t42154, t42212, t42213, t42226, t42228, t42245, t42308, t42309, t42339)
}
