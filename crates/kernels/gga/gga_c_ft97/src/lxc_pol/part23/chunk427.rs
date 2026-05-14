//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 427/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk427<F: Float>(t2681: F, t5225: F, t27: F, t89: F, t1196: F, t284: F, t291: F, t1197: F, t4092: F, t1208: F, t4064: F, t2697: F, t4939: F, t4977: F, t801: F, t274: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5226 = t2681 * t5225;
    let t5228 = t89 * t27 * t5226;
    let t5230 = t1196 * t1196;
    let t5231 = t5230 * t284;
    let t5232 = t5231 * t291;
    let t5234 = t4092 * t1197;
    let t5239 = t4064 * t1208;
    let t5242 = t2697 * t4939;
    let t5245 = t801 * t4977;
    let t5248 = t4939 * t274;
    (t5226, t5228, t5230, t5231, t5232, t5234, t5239, t5242, t5245, t5248)
}
