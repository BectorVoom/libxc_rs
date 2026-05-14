//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 497/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk497<F: Float>(t487: F, t5219: F, t1770: F, t1209: F, t1811: F, t1256: F, t1804: F, t1786: F, t1796: F, t3172: F, t1247: F, t1263: F, t3367: F, t1032: F, t1246: F, t1774: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5220 = t5219 * t487;
    let t5225 = t1770 * t487;
    let t5251 = t1209 * t1811;
    let t5254 = t1804 * t1256;
    let t5256 = t1786 * t1256;
    let t5265 = t3172 * t1796;
    let t5266 = t1247 * t5265;
    let t5268 = t1263 * t3367;
    let t5273 = t1770 * t1032;
    let t5274 = t5273 * t1246;
    let t5277 = t1263 * t1774;
    (t5220, t5225, t5251, t5254, t5256, t5265, t5266, t5268, t5273, t5274, t5277)
}
