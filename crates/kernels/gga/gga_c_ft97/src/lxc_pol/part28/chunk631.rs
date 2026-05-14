//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 631/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk631<F: Float>(t27015: F, t3483: F, t13140: F, t604: F, t6718: F, t379: F, t2210: F, t6696: F, t8392: F, t1369: F, t376: F, t6669: F, t23900: F, t920: F, t1969: F, t446: F) -> (F, F, F, F, F, F, F, F) {
    let t27016 = t27015 * t3483;
    let t27017 = t13140 * t27016;
    let t27020 = t604 * t6718;
    let t27021 = t27020 * t379;
    let t27022 = t2210 * t27021;
    let t27025 = t8392 * t6696;
    let t27028 = t1369 * t376 * t6669;
    let t27030 = t23900 * t920;
    let t27031 = t1969 * t27030;
    let t27032 = t446 * t27031;
    (t27016, t27017, t27021, t27022, t27025, t27028, t27030, t27032)
}
