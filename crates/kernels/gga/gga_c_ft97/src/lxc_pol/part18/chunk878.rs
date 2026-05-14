//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 878/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk878<F: Float>(t1882: F, t5871: F, t1384: F, t2157: F, t2179: F, t144: F, t5968: F, t609: F, t5937: F, t358: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23534 = t1882 * t5871;
    let t23536 = t1384 * t2157;
    let t23537 = t2179 * t23536;
    let t23538 = t144 * t23537;
    let t23541 = t5968 * t609;
    let t23542 = t2179 * t23541;
    let t23543 = t144 * t23542;
    let t23546 = t1882 * t5937;
    let t23548 = t1384 * t358;
    (t23534, t23536, t23537, t23538, t23541, t23542, t23543, t23546, t23548)
}
