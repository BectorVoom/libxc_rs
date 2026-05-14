//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 909/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk909<F: Float>(t20602: F, t442: F, t5971: F, t5964: F, t5965: F, t6: F, t5972: F, t1037: F, t1338: F, t1431: F, t1672: F, t1180: F, t5541: F, t1648: F, t583: F, t14873: F, t169: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20604 = t5971 * t20602 * t442;
    let t20768 = t5964 * t5965 * t6;
    let t20773 = t5972 * t6;
    let t20774 = t5971 * t20773;
    let t20897 = t1037 * t1338;
    let t21049 = t1672 * t1431;
    let t21053 = t5541 * t1180;
    let t21054 = t1648 * t583;
    let t21072 = t169 * t14873;
    (t20604, t20768, t20773, t20774, t20897, t21049, t21053, t21054, t21072)
}
