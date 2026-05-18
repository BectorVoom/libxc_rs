//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 790/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk790<F: Float>(t4831: F, t973: F, t4851: F, t993: F, t4854: F, t7341: F, t2367: F, t5068: F, t999: F, t7501: F, t2418: F, t4814: F) -> (F, F, F, F, F, F, F) {
    let t13733 = t4831 * t973;
    let t13794 = t4851 * t993;
    let t13796 = t7341 * t4854;
    let t13802 = t2367 * t5068;
    let t13803 = t999 * t13802;
    let t13842 = t7501 * t4854;
    let t13890 = t4814 * t2418;
    (t13733, t13794, t13796, t13802, t13803, t13842, t13890)
}
