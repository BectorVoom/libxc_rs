//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 431/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk431<F: Float>(t301: F, t579: F, t336: F, t2046: F, t372: F, t599: F, t578: F, t137: F, t429: F, t128: F, t577: F) -> (F, F, F, F, F, F, F) {
    let t2047 = t579 * t301;
    let t2048 = t336 * t2047;
    let t2049 = t2046 * t2048;
    let t2051 = t599 * t372;
    let t2052 = t336 * t2051;
    let t2053 = t578 * t2052;
    let t2056 = t336 * t429 * t137;
    let t2057 = t578 * t2056;
    let t2059 = t577 * t128;
    (t2048, t2049, t2052, t2053, t2056, t2057, t2059)
}
