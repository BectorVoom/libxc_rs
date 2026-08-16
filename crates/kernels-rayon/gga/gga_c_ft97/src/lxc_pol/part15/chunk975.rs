//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 975/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk975(t21665: f64, t8392: f64, t1882: f64, t21496: f64, t21474: f64, t21657: f64, t21765: f64, t21647: f64, t21652: f64, t21501: f64, t21483: f64, t21479: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t81469 = t8392 * t21665;
    let t81472 = t1882 * t21496;
    let t81478 = t1882 * t21474;
    let t81485 = t8392 * t21657;
    let t81527 = t8392 * t21765;
    let t81545 = t8392 * t21647;
    let t81547 = t8392 * t21652;
    let t81697 = t1882 * t21501;
    let t81721 = t1882 * t21483;
    let t81723 = t1882 * t21479;
    (t81469, t81472, t81478, t81485, t81527, t81545, t81547, t81697, t81721, t81723)
}
