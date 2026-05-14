//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 833/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk833<F: Float>(t1853: F, t22943: F, t1825: F, t5743: F, t1332: F, t8360: F, t1820: F, t5710: F, t5664: F, t92: F) -> (F, F, F, F, F) {
    let t22944 = t22943 * t1853;
    let t22946 = t1825 * t5743;
    let t22948 = t8360 * t1332;
    let t22950 = t5710 * t1820;
    let t22952 = t5664 * t92;
    (t22944, t22946, t22948, t22950, t22952)
}
