//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 909/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk909<F: Float>(t1388: F, t1457: F, t1044: F, t128: F, t188: F, t1386: F, t1642: F, t5963: F, t5973: F, t662: F, t21625: F, t5217: F, t1338: F, t1463: F, t136: F, t4046: F) -> (F, F, F, F, F, F, F) {
    let t21778 = t1388 * t1457;
    let t21801 = t1044 * t188 * t128;
    let t21825 = t1386 * t1642;
    let t21838 = t5963 * t662 * t5973;
    let t21842 = t5963 * t21625 * t5217;
    let t21991 = t1463 * t1338;
    let t22117 = 1.0 / t4046 / t136;
    (t21778, t21801, t21825, t21838, t21842, t21991, t22117)
}
