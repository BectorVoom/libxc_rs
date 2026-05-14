//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 798/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk798<F: Float>(t1735: F, t7632: F, t1750: F, t1795: F, t1775: F, t1868: F, t1680: F, t1872: F, t7839: F, t1804: F, t1823: F, t5218: F, t7514: F, t1648: F, t5545: F, t1403: F, t1407: F, t1663: F) -> (F, F, F, F, F, F, F, F) {
    let t16597 = 16.0 / 5.0 * t7632 * t1735;
    let t16599 = 8.0 / 5.0 * t1750 * t1795;
    let t16601 = 4.0 / 5.0 * t1775 * t1868;
    let t16603 = 8.0 / 5.0 * t1680 * t1868;
    let t16605 = 16.0 / 5.0 * t7839 * t1872;
    let t16609 = 64.0 / 15.0 * t5218 * t7514 * t1804 * t1823;
    let t16611 = 16.0 / 9.0 * t1648 * t5545;
    let t16613 = t1663 * t1407 * t1403;
    (t16597, t16599, t16601, t16603, t16605, t16609, t16611, t16613)
}
