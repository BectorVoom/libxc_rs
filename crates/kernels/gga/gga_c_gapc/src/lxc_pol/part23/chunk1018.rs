//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 1018/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk1018<F: Float>(t1303: F, t137: F, t442: F, t5971: F, t1338: F, t5964: F, t5965: F, t6: F, t5972: F, t1037: F, t1431: F, t1672: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20594 = t1303 * t137;
    let t20596 = t5971 * t20594 * t442;
    let t20602 = t1338 * t137;
    let t20604 = t5971 * t20602 * t442;
    let t20768 = t5964 * t5965 * t6;
    let t20773 = t5972 * t6;
    let t20774 = t5971 * t20773;
    let t20897 = t1037 * t1338;
    let t21049 = t1672 * t1431;
    (t20594, t20596, t20602, t20604, t20768, t20773, t20774, t20897, t21049)
}
