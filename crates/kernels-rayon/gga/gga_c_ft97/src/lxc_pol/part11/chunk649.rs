//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 649/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk649(t9016: f64, t9017: f64, t27: f64, t89: f64, t1984: f64, t2075: f64, t558: f64, t28: f64, t143: f64, t7763: f64, t7765: f64, t7761: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9018 = t9016 * t9017;
    let t9020 = t89 * t27 * t9018;
    let t9022 = t1984 * t558 * t2075;
    let t9024 = t89 * t28 * t9022;
    let t9025 = t143 * t7763;
    let t9026 = t9025 * t7765;
    let t9028 = t89 * t7761 * t9026;
    (t9018, t9020, t9022, t9024, t9025, t9026, t9028)
}
