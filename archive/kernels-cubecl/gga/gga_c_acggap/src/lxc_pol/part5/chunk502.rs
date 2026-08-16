//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 502/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk502<F: Float>(t1670: F, t495: F, t326: F, t38: F, t19: F, t420: F, t128: F, t130: F, t163: F, t167: F, t228: F, t577: F) -> (F, F, F, F, F, F, F, F) {
    let t1946 = t1670 * t495;
    let t1963 = t38 * t326;
    let t1964 = F::cast_from(1.0_f64) / t1963;
    let t1981 = t420 * t19;
    let t1982 = t1981 * t128;
    let t2015 = t130 * t163;
    let t2028 = t167 * t19;
    let t2029 = t2028 * t128;
    let t2035 = t130 * t228;
    let t2059 = t577 * t128;
    (t1946, t1963, t1964, t1982, t2015, t2029, t2035, t2059)
}
