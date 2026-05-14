//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1014/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1014<F: Float>(t2506: F, t31029: F, t1434: F, t193: F, t30986: F, t9770: F, t446: F, t1424: F, t4934: F) -> (F, F, F, F, F) {
    let t31030 = t2506 * t31029;
    let t31032 = t1434 * t193 * t31030;
    let t31033 = t9770 * t30986;
    let t31034 = t446 * t31033;
    let t31036 = t1424 * t4934;
    (t31030, t31032, t31033, t31034, t31036)
}
