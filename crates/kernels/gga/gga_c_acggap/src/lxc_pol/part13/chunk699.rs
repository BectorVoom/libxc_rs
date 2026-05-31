//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 699/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk699<F: Float>(t7488: F, t2061: F, t361: F, t2060: F, t3360: F, t7336: F) -> (F, F, F, F) {
    let t7489 = F::cast_from(0.305625e-1_f64) * t7488;
    let t7490 = t361 * t2061;
    let t7491 = t2060 * t7490;
    let t7493 = t3360 * t7336;
    (t7489, t7490, t7491, t7493)
}
