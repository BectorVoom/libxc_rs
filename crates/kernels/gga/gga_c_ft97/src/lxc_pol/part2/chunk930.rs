//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 930/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk930<F: Float>(t2253: F, t4359: F, t10904: F, t1268: F, t2939: F, t898: F, t12170: F, t4347: F, t1263: F, t8640: F, t2951: F, t4357: F) -> (F, F, F, F, F) {
    let t14423 = F::new(2.0) * t2253 * t4359;
    let t14426 = t898 * t10904 * t1268 * t2939;
    let t14429 = t12170 * t4347;
    let t14431 = t8640 * t1263;
    let t14434 = t898 * t4357 * t2951;
    (t14423, t14426, t14429, t14431, t14434)
}
