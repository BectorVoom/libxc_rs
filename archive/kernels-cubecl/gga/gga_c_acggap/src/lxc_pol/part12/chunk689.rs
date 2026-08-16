//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 689/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk689<F: Float>(t599: F, t922: F, t142: F, t7450: F, t1979: F, t381: F) -> (F, F, F, F) {
    let t7451 = t599 * t922;
    let t7452 = t142 * t7451;
    let t7453 = t7450 * t7452;
    let t7457 = t381 * t1979;
    (t7451, t7452, t7453, t7457)
}
