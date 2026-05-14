//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 480/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk480<F: Float>(t2214: F, t530: F, t514: F, t1632: F, t481: F, t551: F, t566: F, t489: F, t525: F, t524: F, t1543: F, t506: F, t529: F, t119: F, t1266: F, t122: F, t507: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t2215 = t2214 * t530;
    let t2216 = t514 * t2215;
    let t2218 = t1632 * t481;
    let t2219 = t551 * t2218;
    let t2220 = t566 * t2219;
    let t2222 = t525 * t489;
    let t2223 = t524 * t2222;
    let t2224 = t506 * t1543;
    let t2225 = t529 * t2224;
    let t2228 = t1266 * t119;
    let t2231 = 0.16463622957338778997e-1 * t2228 * t122 * t507;
    (t2215, t2216, t2218, t2219, t2220, t2222, t2223, t2224, t2225, t2228, t2231)
}
