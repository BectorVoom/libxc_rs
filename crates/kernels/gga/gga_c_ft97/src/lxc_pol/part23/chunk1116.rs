//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1116/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1116<F: Float>(t108083: F, t27742: F, t668: F, t1882: F, t27470: F, t1424: F, t9895: F, t24543: F, t27789: F, t6896: F, t8232: F, t2360: F, t6837: F, t27856: F, t6109: F, t681: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t108084 = 2.0 / 3.0 * t108083;
    let t108109 = t27742 * t668;
    let t108114 = t1882 * t27470;
    let t108115 = 2.0 / 9.0 * t108114;
    let t108120 = t9895 * t1424;
    let t108138 = t24543 * t27789;
    let t108139 = t108138 / 9.0;
    let t108140 = t8232 * t6896;
    let t108142 = t6837 * t2360;
    let t108157 = t6109 * t681 * t27856;
    (t108084, t108109, t108114, t108115, t108120, t108138, t108139, t108140, t108142, t108157)
}
