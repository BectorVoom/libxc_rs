//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 311/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk311<F: Float>(t1852: F, t1853: F, t83: F, t379: F, t447: F, t499: F, t110: F, t1651: F, t1642: F, t82: F) -> (F, F, F, F) {
    let t1854 = t1852 * t1853;
    let t1855 = t83 * t1854;
    let t1859 = t447 * t499 * t379;
    let t1863 = t447 * t110 * t1651;
    let t1866 = t1642 * t82;
    (t1855, t1859, t1863, t1866)
}
