//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 546/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk546<F: Float>(t5205: F, t5206: F, t1196: F, t3358: F, t3546: F, t5044: F, t5049: F, t5054: F, t5058: F, t459: F, t1208: F, t1769: F, t487: F, t1770: F, t1214: F, t1774: F) -> (F, F, F, F, F, F) {
    let t5207 = t5205 * t5206;
    let t5209 = 0.17315859105681463759e2 * t1196 * t5207;
    let t5215 = t3546 - 0.27777777777777777778e-2 * t3358 - 0.27777777777777777778e-2 * t5044 - 0.55555555555555555555e-2 * t5049 + 0.16666666666666666667e-1 * t5054 + 0.83333333333333333333e-2 * t5058;
    let t5216 = t5215 * t459;
    let t5219 = t1769 * t1208;
    let t5220 = t5219 * t487;
    let t5225 = t1770 * t487;
    let t5230 = t1774 * t1214;
    (t5209, t5216, t5219, t5220, t5225, t5230)
}
