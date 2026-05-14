//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 724/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk724<F: Float>(t2057: F, t4710: F, t3355: F, t3404: F, t4711: F, t542: F, t131: F, t4673: F, t139: F, t1013: F, t51: F, t6: F, t398: F, t4702: F, t8907: F, t12449: F, t12452: F, t16763: F, t16769: F, t2001: F, t3392: F, t3393: F, t399: F, t4675: F, t4712: F, t538: F, t554: F) -> (F,) {
    let t16773 = t2057 * t4710;
    let t16777 = t3355 * t3404;
    let t16780 = t542 * t4711;
    let t16785 = t4673 * t131;
    let t16786 = t16785 * t139;
    let t16792 = t1013 * t6 * t51;
    let t16793 = t16792 * t398;
    let t16798 = t8907 * t4702;
    let t16802 = 8.0 * t2001 * t16763 + 4.0 * t3392 * t3393 * t3404 - 2.0 * t2001 * t16769 * t538 + 2.0 * t3392 * t16773 * t554 - 4.0 * t2001 * t16777 + 0.60409133884038297798e0 * t16780 * t399 - 0.60409133884038297798e0 * t4712 * t399 - 0.1208182677680765956e1 * t16786 * t399 + 0.1208182677680765956e1 * t4675 * t399 + 0.24163653553615319119e1 * t12449 * t16793 - 0.24163653553615319119e1 * t12452 * t16793 - 6.0 * t3392 * t16798 * t554;
    (t16802,)
}
