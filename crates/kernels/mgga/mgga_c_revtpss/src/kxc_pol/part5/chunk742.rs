//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 742/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk742<F: Float>(t1256: F, t1804: F, t1786: F, t1230: F, t1803: F, t225: F, t5216: F, t480: F, t1796: F, t3172: F, t1247: F, t1263: F, t3367: F, t4181: F, t1042: F, t1032: F, t1770: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5254 = t1804 * t1256;
    let t5256 = t1786 * t1256;
    let t5258 = t1230 * t1803;
    let t5261 = t5216 * t225;
    let t5262 = t5261 * t480;
    let t5265 = t3172 * t1796;
    let t5266 = t1247 * t5265;
    let t5268 = t1263 * t3367;
    let t5269 = t5268 * t4181;
    let t5270 = t1042 * t5269;
    let t5273 = t1770 * t1032;
    (t5254, t5256, t5258, t5261, t5262, t5265, t5266, t5268, t5269, t5270, t5273)
}
