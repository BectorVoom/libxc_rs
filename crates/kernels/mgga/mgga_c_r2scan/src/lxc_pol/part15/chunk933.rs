//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 933/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk933<F: Float>(t10710: F, t6476: F, t10728: F, t3344: F, t776: F, t2096: F, t269: F, t23: F, t39: F, t6077: F, t255: F, t6321: F) -> (F, F, F, F, F, F) {
    let t10729 = t10710 * t6476;
    let t10730 = t10728 * t10729;
    let t10732 = t776 * t3344;
    let t10734 = t2096 * t269;
    let t10737 = F::cast_from(1.0_f64) / t23 / t6077 / t39;
    let t10740 = t10734 * t10737 * t255 * t6321;
    (t10729, t10730, t10732, t10734, t10737, t10740)
}
