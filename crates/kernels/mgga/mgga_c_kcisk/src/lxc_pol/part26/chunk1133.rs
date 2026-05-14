//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1133/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1133<F: Float>(t26035: F, t32013: F, t6204: F, t1327: F, t2168: F, t32069: F, t1322: F, t2059: F, t32088: F, t3937: F, t20160: F, t9795: F) -> (F, F, F, F, F, F, F, F) {
    let t33433 = t32013 * t26035;
    let t33434 = t6204 * t33433;
    let t33437 = t2168 * t1327;
    let t33438 = t32069 * t33437;
    let t33439 = t6204 * t33438;
    let t33444 = t2059 * t1322;
    let t33445 = t32088 * t33444;
    let t33446 = t3937 * t33445;
    let t33451 = t20160 * t9795;
    (t33433, t33434, t33437, t33438, t33439, t33445, t33446, t33451)
}
