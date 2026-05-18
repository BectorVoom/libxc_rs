//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 296/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk296<F: Float>(t1196: F, t291: F, t800: F, t281: F, t283: F, t1121: F, t1125: F, t818: F) -> (F, F, F, F, F) {
    let t1197 = t291 * t1196;
    let t1198 = t800 * t1197;
    let t1200 = t281 * t283;
    let t1201 = t1200 * t291;
    let t1208 = -F::new(0.13335600218518518519e0) * t1121 + t818 + F::new(0.16669500273148148149e-1) * t1125;
    (t1197, t1198, t1200, t1201, t1208)
}
