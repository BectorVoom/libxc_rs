//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 711/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk711<F: Float>(t2624: F, t9800: F, t9829: F, t2617: F, t3255: F, t7803: F, t2679: F, t3251: F, t9805: F, t9893: F, t28023: F, t883: F, t1967: F, t5641: F, t28640: F, t28641: F, t28668: F) -> (F, F, F, F, F, F, F) {
    let t40969 = t9800 * t2624 * t9829;
    let t40986 = t7803 * t3255 * t2617;
    let t40989 = t9800 * t3251 * t2679;
    let t41008 = t9805 * t9893 * t2679;
    let t41010 = t883 * t28023;
    let t41012 = t9800 * t1967 * t41010;
    let t41015 = t9805 * t5641 * t41010;
    let t41019 = t28640 * t28641 * t883 * t28668;
    (t40969, t40986, t40989, t41008, t41012, t41015, t41019)
}
