//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1329/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1329<F: Float>(t23657: F, t23671: F, t27086: F, t379: F, t27185: F, t376: F, t89: F, t1017: F, t28: F, t94155: F, t2075: F, t26791: F, t104235: F, t1986: F, t1643: F, t27072: F, t6656: F) -> (F, F, F, F, F, F, F) {
    let t105633 = t23657 * t23671 * t27086 * t379;
    let t105637 = t89 * t376 * t27185;
    let t105638 = 4.0 / 3.0 * t105637;
    let t105641 = t89 * t28 * t94155 * t1017;
    let t105645 = t89 * t28 * t26791 * t2075;
    let t105649 = t89 * t28 * t104235 * t1986;
    let t105653 = t23657 * t27072 * t6656 * t1643;
    (t105633, t105637, t105638, t105641, t105645, t105649, t105653)
}
