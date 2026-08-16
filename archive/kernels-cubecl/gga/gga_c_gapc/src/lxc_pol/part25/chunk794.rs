//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 794/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk794<F: Float>(t3028: F, t677: F, t3021: F, t5075: F, t1043: F, t144: F, t5979: F, t8820: F, t5977: F, t1679: F, t3016: F, t3013: F, t5252: F) -> (F, F, F, F, F, F) {
    let t9097 = t3028 * t677;
    let t9099 = t3021 * t5075;
    let t9100 = t1043 * t9099;
    let t9103 = t8820 * t144 * t5979;
    let t9104 = t5977 * t9103;
    let t9106 = t3016 * t1679;
    let t9108 = t5252 * t3013;
    (t9097, t9099, t9100, t9104, t9106, t9108)
}
