//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 945/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk945<F: Float>(t4973: F, t724: F, t773: F, t18123: F, t265: F, t2594: F, t4965: F, t1091: F, t4005: F, t4934: F, t766: F, t2574: F, t762: F) -> (F, F, F, F, F) {
    let t18602 = t724 * t773 * t4973;
    let t18606 = t724 * t265 * t18123;
    let t18610 = t2594 * t773 * t4965;
    let t18614 = t724 * t4005 * t1091;
    let t18617 = t4934 * t766;
    let t18619 = t2574 * t762 * t18617;
    (t18602, t18606, t18610, t18614, t18619)
}
