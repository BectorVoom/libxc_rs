//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 497/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk497<F: Float>(t326: F, t38: F, t19: F, t420: F, t128: F, t130: F, t163: F, t167: F, t228: F, t577: F, t56: F, t137: F, t495: F, t506: F, t6: F, t119: F, t182: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t1963 = t38 * t326;
    let t1964 = 1.0 / t1963;
    let t1981 = t420 * t19;
    let t1982 = t1981 * t128;
    let t2015 = t130 * t163;
    let t2028 = t167 * t19;
    let t2029 = t2028 * t128;
    let t2035 = t130 * t228;
    let t2059 = t577 * t128;
    let t2066 = t56 * t420;
    let t2297 = t137 * t495;
    let t2325 = t6 * t506;
    let t2450 = t119 * t182;
    (t1963, t1964, t1982, t2015, t2029, t2035, t2059, t2066, t2297, t2325, t2450)
}
