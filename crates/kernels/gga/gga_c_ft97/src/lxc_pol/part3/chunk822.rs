//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 822/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk822<F: Float>(t1013: F, t51: F, t6: F, t398: F, t4702: F, t8907: F, t12449: F, t12452: F, t16763: F, t16769: F, t16773: F, t16777: F, t16780: F, t16786: F, t2001: F, t3392: F, t3393: F, t3404: F, t399: F, t4675: F, t4712: F, t538: F, t554: F) -> F {
    let t16792 = t1013 * t6 * t51;
    let t16793 = t16792 * t398;
    let t16798 = t8907 * t4702;
    let t16802 = F::new(8.0) * t2001 * t16763 + F::new(4.0) * t3392 * t3393 * t3404 - F::new(2.0) * t2001 * t16769 * t538 + F::new(2.0) * t3392 * t16773 * t554 - F::new(4.0) * t2001 * t16777 + F::new(0.60409133884038297798e0) * t16780 * t399 - F::new(0.60409133884038297798e0) * t4712 * t399 - F::new(0.1208182677680765956e1) * t16786 * t399 + F::new(0.1208182677680765956e1) * t4675 * t399 + F::new(0.24163653553615319119e1) * t12449 * t16793 - F::new(0.24163653553615319119e1) * t12452 * t16793 - F::new(6.0) * t3392 * t16798 * t554;
    t16802
}
