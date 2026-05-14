//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1060/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1060<F: Float>(t2459: F, t6837: F, t1434: F, t193: F, t2506: F, t3704: F, t6135: F, t743: F, t27742: F, t668: F, t2354: F, t446: F, t505: F, t1882: F, t27470: F, t24437: F, t24546: F, t2574: F, t27796: F) -> (F, F, F, F, F, F, F) {
    let t108101 = t6837 * t2459;
    let t108104 = t1434 * t193 * t2506 * t108101;
    let t108107 = t1434 * t3704 * t743 * t6135;
    let t108109 = t27742 * t668;
    let t108112 = t446 * t2354 * t108109 * t505;
    let t108114 = t1882 * t27470;
    let t108115 = 2.0 / 9.0 * t108114;
    let t108118 = t24437 * t2574 * t24546 * t27796;
    (t108101, t108104, t108107, t108112, t108114, t108115, t108118)
}
