//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 713/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk713<F: Float>(t13526: F, t3724: F, t1092: F, t1771: F, t3740: F, t458: F, t3743: F, t11176: F, t3747: F, t222: F, t226: F, t1113: F, t236: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13527 = t3724 * t13526;
    let t13538 = t1771 * t1092;
    let t13540 = t458 * t3740;
    let t13541 = F::new(4.0) / F::new(27.0) * t13540;
    let t13542 = t458 * t3743;
    let t13543 = F::new(4.0) / F::new(9.0) * t13542;
    let t13544 = t11176 * t3747;
    let t13580 = t222 * t226;
    let t13581 = t236 * t1113;
    (t13527, t13538, t13540, t13541, t13542, t13543, t13544, t13580, t13581)
}
