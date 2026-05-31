//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1045/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1045<F: Float>(t14431: F, t13925: F, t500: F, t8: F, t1697: F, t51: F, t49: F, t75: F, t10: F, t47: F, t204: F, t5401: F, t58: F) -> (F, F, F, F, F, F, F) {
    let t16089 = F::cast_from(1.0_f64) / t14431;
    let t16111 = F::cast_from(1.0_f64) / t13925;
    let t16129 = t8 * t500;
    let t16190 = t51 * t1697;
    let t16193 = F::cast_from(0.11483599538271604938e-1_f64) * t49 * t16190 * t75;
    let t16194 = t47 * t10;
    let t16200 = F::cast_from(1.0_f64) / t58 / t16194 * t47 * t5401 * t204 / F::cast_from(48.0_f64);
    (t16089, t16111, t16129, t16190, t16193, t16194, t16200)
}
