//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 268/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk268<F: Float>(t488: F, t979: F, t83: F, t28: F, t442: F, t446: F, t89: F, t951: F, t955: F, t973: F, t103: F, t971: F) -> (F, F, F, F) {
    let t980 = t488 * t979;
    let t981 = t83 * t980;
    let t984 = -t442 - t446 * t951 / F::new(9.0) - t446 * t955 / F::new(3.0) + t89 * t28 * t973 / F::new(3.0) - t446 * t981 / F::new(3.0);
    let t986 = t971 * t103;
    (t980, t981, t984, t986)
}
