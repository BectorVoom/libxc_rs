//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1098/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1098<F: Float>(t1611: F, t8: F, t1608: F, t5566: F, t22675: F, t25: F, t1685: F, t37: F, t78: F, t401: F, t66: F, t22679: F, t1669: F, t93046: F, t22582: F, t8042: F) -> (F, F, F, F, F, F, F) {
    let t93076 = t8 * t1611;
    let t93078 = t1608 * t5566 * t93076;
    let t93084 = t22675 * t25;
    let t93092 = t37 * t1685 * t78;
    let t93102 = t401 * t66;
    let t93106 = t22679 * t66;
    let t93117 = t1669 * t93046;
    let t93122 = t8042 * t22582;
    (t93078, t93084, t93092, t93102, t93106, t93117, t93122)
}
