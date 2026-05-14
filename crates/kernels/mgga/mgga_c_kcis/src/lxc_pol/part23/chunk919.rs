//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 919/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk919<F: Float>(t26473: F, t26513: F, t7607: F, t782: F, t826: F, t2165: F, t228: F, t26402: F, t26411: F, t26418: F, t26421: F, t26422: F, t26425: F, t2766: F, t2772: F, t2789: F, t7657: F, t7669: F, t9007: F, t9017: F, t906: F) -> (F, F, F, F) {
    let t26514 = t26473 + t26513;
    let t26516 = t7607 * t782;
    let t26517 = t26516 * t826;
    let t26518 = 2.0 * t26517;
    let t26519 = -t2165 * t9007 + t228 * t26514 + 2.0 * t26411 * t2772 - 2.0 * t26422 * t906 - 6.0 * t26425 * t9017 - 2.0 * t2766 * t7669 - t2789 * t7657 - t26402 - t26418 - t26421 + t26518;
    (t26514, t26516, t26517, t26519)
}
