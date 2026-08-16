//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1825/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1825<F: Float>(t4343: F, t890: F, t1544: F, t2408: F, t4537: F, t775: F, t2832: F, t2411: F, t14365: F, t1448: F, t5591: F, t1868: F, t4144: F) -> (F, F, F, F, F, F, F, F) {
    let t61102 = t4343 * t890;
    let t61155 = t1544 * t2408;
    let t61182 = t775 * t4537;
    let t61203 = t1544 * t2832;
    let t63164 = t4537 * t890;
    let t63185 = t2411 * t1544;
    let t63186 = t63185 * t14365;
    let t73394 = t5591 * t1448;
    let t73488 = t1868 * t4144;
    (t61102, t61155, t61182, t61203, t63164, t63186, t73394, t73488)
}
