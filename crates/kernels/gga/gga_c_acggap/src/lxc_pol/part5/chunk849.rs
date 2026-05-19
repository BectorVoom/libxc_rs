//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 849/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk849<F: Float>(t2981: F, t883: F, t11795: F, t11797: F, t11800: F, t11803: F, t11806: F, t11811: F, t11813: F, t11815: F, t11817: F, t11820: F) -> (F, F) {
    let t11922 = t883 * t2981;
    let t11934 = -F::cast_from(0.28769444444444444444e1_f64) * t11795 + F::cast_from(0.27618666666666666667e2_f64) * t11797 - F::cast_from(0.10229135802469135803e2_f64) * t11800 + F::cast_from(0.89504938271604938273e1_f64) * t11803 + F::cast_from(0.31310740740740740741e1_f64) * t11806 + F::new(0.366775e-1) * t11811 - F::new(0.58684e0) * t11813 + F::cast_from(0.65204444444444444445e0_f64) * t11815 + F::cast_from(0.5705388888888888889e0_f64) * t11817 + F::cast_from(0.13490888888888888889e1_f64) * t11820;
    (t11922, t11934)
}
