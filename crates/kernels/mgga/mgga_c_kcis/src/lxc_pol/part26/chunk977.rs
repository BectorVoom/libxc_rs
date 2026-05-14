//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 977/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk977<F: Float>(t1014: F, t7928: F, t27348: F, t7898: F, t1458: F, t1466: F, t2244: F, t3245: F, t110: F, t2238: F, t2237: F, t1505: F, t7938: F, t2247: F, t4188: F, t4248: F, t491: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t27462 = t1014 * t7928;
    let t27471 = t7898 * t27348;
    let t27475 = t1458 * t1466;
    let t27482 = t3245 * t2244;
    let t27483 = 0.55273148148148148147e-3 * t27482;
    let t27484 = t110 * t2238;
    let t27486 = 0.15445601851851851852e-3 * t2237 * t27484;
    let t27491 = t7938 * t1505;
    let t27494 = t2247 * t4188;
    let t27514 = t4248 * t491;
    (t27462, t27471, t27475, t27482, t27483, t27484, t27486, t27491, t27494, t27514)
}
