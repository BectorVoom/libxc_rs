//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1111/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1111<F: Float>(t109322: F, t1434: F, t193: F, t743: F, t2373: F, t42500: F, t446: F, t6852: F, t2354: F, t24546: F, t27805: F, t3746: F, t1424: F, t42123: F, t13757: F, t1901: F) -> (F, F, F, F) {
    let t109325 = t1434 * t193 * t743 * t109322;
    let t109329 = t446 * t42500 * t6852 * t2373;
    let t109333 = t27805 * t2354 * t24546 * t3746;
    let t109335 = t42123 * t1424;
    let t109337 = t1901 * t109335 * t13757;
    (t109325, t109329, t109333, t109337)
}
