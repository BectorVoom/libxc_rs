//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 821/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk821<F: Float>(t12401: F, t16762: F, t4710: F, t549: F, t2057: F, t3355: F, t3404: F, t4711: F, t542: F, t131: F, t4673: F, t139: F) -> (F, F, F, F, F, F) {
    let t16763 = t12401 * t16762;
    let t16769 = t549 * t4710;
    let t16773 = t2057 * t4710;
    let t16777 = t3355 * t3404;
    let t16780 = t542 * t4711;
    let t16785 = t4673 * t131;
    let t16786 = t16785 * t139;
    (t16763, t16769, t16773, t16777, t16780, t16786)
}
