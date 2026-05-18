//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 930/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk930<F: Float>(t22914: F, t32013: F, t32425: F, t108: F, t32325: F, t458: F, t7161: F, t5504: F, t1337: F, t5617: F, t1286: F, t32402: F, t376: F) -> (F, F, F, F, F, F, F) {
    let t135994 = t22914 * t32013;
    let t135996 = t22914 * t32425;
    let t136000 = t32325 * t108;
    let t136015 = t7161 * t458;
    let t136016 = t136015 * t5504;
    let t136018 = t5617 * t1337;
    let t136037 = t1286 * t376 * t32402;
    (t135994, t135996, t136000, t136015, t136016, t136018, t136037)
}
