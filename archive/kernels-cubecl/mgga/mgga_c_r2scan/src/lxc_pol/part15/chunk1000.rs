//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1000/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1000<F: Float>(t261: F, t2720: F, t3299: F, t10879: F, t3594: F, t2661: F, t3304: F, t545: F, t979: F, t3300: F, t2206: F, t978: F) -> (F, F, F, F, F, F, F, F) {
    let t11736 = t261 * t2720;
    let t11737 = t3299 * t11736;
    let t11739 = t10879 * t3594;
    let t11741 = t261 * t2661;
    let t11742 = t3304 * t11741;
    let t11744 = t545 * t979;
    let t11745 = t11744 * t3300;
    let t11747 = t2206 * t978;
    (t11736, t11737, t11739, t11741, t11742, t11744, t11745, t11747)
}
