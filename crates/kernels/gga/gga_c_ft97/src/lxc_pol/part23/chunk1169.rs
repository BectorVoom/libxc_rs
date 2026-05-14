//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1169/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1169<F: Float>(t28778: F, t99312: F, t2: F, t28719: F, t28743: F, t1882: F, t28793: F, t28535: F, t7080: F, t8232: F, t1486: F, t28800: F, t681: F, t12001: F, t28789: F, t2399: F, t7075: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t113226 = t99312 * t28778;
    let t113227 = t113226 / 3.0;
    let t113238 = t2 * t28719;
    let t113243 = t99312 * t28743;
    let t113244 = t113243 / 3.0;
    let t113249 = t1882 * t28793;
    let t113250 = 4.0 * t113249;
    let t113251 = t1882 * t28535;
    let t113252 = 2.0 / 9.0 * t113251;
    let t113253 = t8232 * t7080;
    let t113268 = t1486 * t681 * t28800;
    let t113269 = t113268 / 3.0;
    let t113270 = t12001 * t28789;
    let t113273 = t1486 * t2399 * t7075;
    (t113226, t113227, t113238, t113243, t113244, t113249, t113250, t113251, t113252, t113253, t113268, t113269, t113270, t113273)
}
