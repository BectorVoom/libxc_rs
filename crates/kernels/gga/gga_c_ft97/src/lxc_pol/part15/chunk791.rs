//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 791/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk791<F: Float>(t21453: F, t2493: F, t21457: F, t21442: F, t9916: F, t21181: F, t9920: F, t2486: F, t21570: F, t21573: F, t21577: F, t21581: F, t21585: F, t21589: F, t21592: F, t462: F, t92: F) -> (F, F, F, F, F, F) {
    let t21595 = t2493 * t21453;
    let t21597 = t2493 * t21457;
    let t21599 = t9916 * t21442;
    let t21602 = t9920 * t21181;
    let t21603 = t2486 * t21602;
    let t21606 = -t92 * t21570 - t462 * t21573 / F::cast_from(3.0_f64) - F::cast_from(6.0_f64) * t92 * t21577 + F::cast_from(6.0_f64) * t462 * t21581 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t462 * t21585 - F::cast_from(2.0_f64) * t462 * t21589 + F::cast_from(2.0_f64) * t462 * t21592 + t462 * t21595 + t462 * t21597 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t462 * t21599 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t462 * t21603;
    (t21595, t21597, t21599, t21602, t21603, t21606)
}
