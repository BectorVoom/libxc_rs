//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 850/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk850<F: Float>(t1063: F, t2229: F, t2453: F, t3259: F, t2436: F, t3234: F, t3240: F, t2452: F, t3239: F, t6773: F, t3258: F, t2437: F) -> (F, F, F, F, F, F) {
    let t10187 = t2229 * t1063;
    let t10189 = t2453 * t3259;
    let t10191 = t3234 * t2436;
    let t10192 = t10191 * t3240;
    let t10194 = t3234 * t2452;
    let t10195 = t10194 * t3259;
    let t10197 = t3239 * t6773;
    let t10198 = t3258 * t10197;
    let t10201 = t2437 * t3240;
    (t10187, t10189, t10192, t10195, t10198, t10201)
}
