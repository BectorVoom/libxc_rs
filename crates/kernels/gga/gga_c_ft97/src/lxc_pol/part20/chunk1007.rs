//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1007/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1007<F: Float>(t6119: F, t9895: F, t24543: F, t24548: F, t1424: F, t9577: F, t1434: F, t24512: F, t681: F, t6137: F, t8232: F, t1882: F, t24521: F, t2399: F, t6128: F, t24395: F, t668: F) -> (F, F, F, F, F, F, F, F) {
    let t96935 = t9895 * t6119;
    let t96940 = t24543 * t24548;
    let t96945 = t1424 * t9577;
    let t96951 = t1434 * t681 * t24512;
    let t96953 = t8232 * t6137;
    let t96955 = t1882 * t24521;
    let t96958 = t1434 * t2399 * t6128;
    let t96960 = t24395 * t668;
    (t96935, t96940, t96945, t96951, t96953, t96955, t96958, t96960)
}
