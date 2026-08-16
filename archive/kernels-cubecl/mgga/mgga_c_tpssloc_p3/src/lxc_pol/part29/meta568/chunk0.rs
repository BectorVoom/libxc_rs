//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1985/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1985<F: Float>(t1509: F, t2678: F, t1484: F, t2631: F, t9975: F, t2710: F, t4233: F, t852: F, t13170: F, t252: F, t1519: F, t13068: F, t225: F) -> (F, F, F, F, F, F, F, F, F) {
    let t46693 = t1509 * t2678;
    let t47012 = t1484 * t2631;
    let t47262 = t1509 * t2631;
    let t47285 = t1509 * t9975;
    let t47425 = t2710 * t1509;
    let t47439 = t852 * t4233;
    let t47448 = t252 * t13170;
    let t47528 = t1519 * t2678;
    let t47568 = t13068 * t225;
    (t46693, t47012, t47262, t47285, t47425, t47439, t47448, t47528, t47568)
}
