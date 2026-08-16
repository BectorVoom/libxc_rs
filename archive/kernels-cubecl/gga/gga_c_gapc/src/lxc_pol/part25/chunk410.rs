//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 410/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk410<F: Float>(t132: F, t762: F, t737: F, t88: F, t256: F, t62: F, t748: F, t1150: F, t19: F, t252: F, t1: F, t348: F, t745: F) -> (F, F, F, F, F, F) {
    let t2046 = t132 * t762;
    let t2053 = t88 * t737;
    let t2056 = t256 * t256;
    let t2057 = F::cast_from(1.0_f64) / t2056;
    let t2058 = t62 * t2057;
    let t2059 = t748 * t748;
    let t2063 = t1150 * t252 * t19;
    let t2067 = t348 * t745 * t1;
    (t2046, t2053, t2058, t2059, t2063, t2067)
}
