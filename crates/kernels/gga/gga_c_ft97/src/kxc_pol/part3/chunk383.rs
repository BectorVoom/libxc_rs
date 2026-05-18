//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 383/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk383<F: Float>(t2347: F, t2440: F, t2360: F, t703: F, t754: F, t761: F, t250: F, t251: F, t1771: F, t249: F, t1775: F, t740: F) -> (F, F, F, F, F, F) {
    let t2441 = t2440 * t2347;
    let t2446 = t703 * t2360;
    let t2469 = t754 * t761;
    let t2475 = F::new(1.0) / t251 / t250;
    let t2481 = F::new(4.0) / F::new(9.0) * t1771 * t249;
    let t2482 = t1775 * t740;
    (t2441, t2446, t2469, t2475, t2481, t2482)
}
