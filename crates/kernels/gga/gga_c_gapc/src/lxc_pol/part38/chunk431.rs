//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 431/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk431<F: Float>(t2300: F, t772: F, t132: F, t268: F, t770: F, t798: F, t2216: F, t793: F, t297: F, t966: F, t875: F) -> (F, F, F, F, F, F) {
    let t2301 = t772 * t2300;
    let t2304 = t132 * t268;
    let t2305 = t2304 * t770;
    let t2308 = t2304 * t798;
    let t2311 = t2216 * t793;
    let t2314 = t297 * t966;
    let t2315 = t875 * t875;
    (t2301, t2305, t2308, t2311, t2314, t2315)
}
