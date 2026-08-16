//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 853/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk853<F: Float>(t10226: F, t3255: F, t268: F, t8350: F, t2208: F, t6181: F, t6201: F, t3235: F, t3250: F, t1004: F, t2152: F, t827: F) -> (F, F, F, F, F, F) {
    let t10227 = t10226 * t3255;
    let t10229 = t8350 * t268;
    let t10230 = t10229 * t2208;
    let t10231 = t6181 * t6201;
    let t10232 = t10230 * t10231;
    let t10234 = t3235 * t3250;
    let t10236 = t1004 * t2152;
    let t10237 = t10236 * t827;
    (t10227, t10229, t10230, t10232, t10234, t10237)
}
