//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 327/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk327<F: Float>(t1327: F, t1340: F, t966: F, t968: F, t975: F, t977: F) -> (F, F) {
    let t1422 = -t966 - F::cast_from(0.19388333333333333333e1_f64) * t1327 - t968 - F::cast_from(0.12315e-2_f64) * t1340;
    let t1426 = -t975 - F::cast_from(0.72691666666666666667e3_f64) * t1327 - t977 - F::cast_from(0.78666666666666666667e2_f64) * t1340;
    (t1422, t1426)
}
