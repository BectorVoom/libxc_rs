//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 591/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk591<F: Float>(t304: F, t4922: F, t355: F, t360: F, t303: F, t1699: F, t2880: F, t991: F, t2888: F, t291: F) -> (F, F, F, F, F, F, F) {
    let t4923 = t304 * t4922;
    let t4924 = t4923 * t355;
    let t4925 = t4924 * t360;
    let t4926 = t303 * t4925;
    let t4936 = t2880 * t1699;
    let t4937 = t991 * t4936;
    let t4939 = t2888 * t291;
    (t4923, t4924, t4925, t4926, t4936, t4937, t4939)
}
