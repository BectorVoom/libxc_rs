//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1061/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1061<F: Float>(t21293: F, t6333: F, t1483: F, t6351: F, t2267: F, t4297: F, t19886: F, t4204: F, t6331: F, t14581: F, t6357: F, t2259: F, t4241: F, t14399: F, t2263: F, t1517: F, t6344: F) -> (F, F, F, F, F, F, F, F, F) {
    let t21294 = t21293 * t6333;
    let t21296 = t1483 * t6351;
    let t21298 = t4297 * t2267;
    let t21300 = t4204 * t19886;
    let t21301 = t6331 * t21300;
    let t21303 = t14581 * t6357;
    let t21306 = t2259 * t4241;
    let t21308 = t14399 * t2263;
    let t21310 = t6344 * t1517;
    (t21294, t21296, t21298, t21300, t21301, t21303, t21306, t21308, t21310)
}
