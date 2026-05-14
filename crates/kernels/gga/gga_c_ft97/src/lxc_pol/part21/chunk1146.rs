//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1146/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1146<F: Float>(t22952: F, t22953: F, t26011: F, t925: F, t4417: F, t473: F, t23031: F, t25955: F, t25985: F, t16241: F, t5691: F, t22958: F, t5674: F, t16155: F, t16150: F, t22986: F) -> (F, F, F, F, F, F, F, F, F) {
    let t116270 = t22952 * t22953 * t26011 * t925;
    let t116272 = t4417 * t473;
    let t116275 = t22952 * t22953 * t23031 * t116272;
    let t116279 = t22952 * t22953 * t25955 * t25985;
    let t116281 = t5691 * t16241;
    let t116283 = t5674 * t22958 * t116281;
    let t116285 = t23031 * t16155;
    let t116287 = t5674 * t22958 * t116285;
    let t116289 = t22986 * t16150;
    (t116270, t116272, t116275, t116279, t116281, t116283, t116285, t116287, t116289)
}
