//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 849/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk849<F: Float>(t10710: F, t6476: F, t10728: F, t3344: F, t776: F, t2096: F, t269: F, t23: F, t39: F, t6077: F, t255: F, t6321: F, t254: F, t120: F, t2176: F, t531: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10729 = t10710 * t6476;
    let t10730 = t10728 * t10729;
    let t10732 = t776 * t3344;
    let t10734 = t2096 * t269;
    let t10737 = 1.0 / t23 / t6077 / t39;
    let t10740 = t10734 * t10737 * t255 * t6321;
    let t10741 = t254 * t10740;
    let t10742 = 0.15573871527278325618e-1 * t10741;
    let t10743 = t120 * t2176;
    let t10744 = t10743 * t531;
    (t10729, t10730, t10732, t10734, t10737, t10740, t10742, t10743, t10744)
}
