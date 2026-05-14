//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1004/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1004<F: Float>(t24211: F, t5996: F, t1403: F, t24183: F, t681: F, t24191: F, t683: F, t2371: F, t24395: F, t1424: F, t2617: F, t2399: F, t6010: F, t24178: F, t10051: F, t1443: F) -> (F, F, F, F, F, F, F, F) {
    let t96782 = t5996 * t24211;
    let t96796 = t1403 * t681 * t24183;
    let t96798 = t683 * t24191;
    let t96808 = t2371 * t24395;
    let t96812 = t1424 * t2617;
    let t96818 = t1403 * t2399 * t6010;
    let t96820 = t5996 * t24178;
    let t96834 = t1443 * t10051;
    (t96782, t96796, t96798, t96808, t96812, t96818, t96820, t96834)
}
