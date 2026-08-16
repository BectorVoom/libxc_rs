//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1071/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1071<F: Float>(t1014: F, t7928: F, t27348: F, t7898: F, t1458: F, t1466: F, t2244: F, t3245: F, t110: F, t2238: F, t2237: F, t1505: F, t7938: F) -> (F, F, F, F, F, F, F, F) {
    let t27462 = t1014 * t7928;
    let t27471 = t7898 * t27348;
    let t27475 = t1458 * t1466;
    let t27482 = t3245 * t2244;
    let t27483 = F::cast_from(0.55273148148148148147e-3_f64) * t27482;
    let t27484 = t110 * t2238;
    let t27486 = F::cast_from(0.15445601851851851852e-3_f64) * t2237 * t27484;
    let t27491 = t7938 * t1505;
    (t27462, t27471, t27475, t27482, t27483, t27484, t27486, t27491)
}
