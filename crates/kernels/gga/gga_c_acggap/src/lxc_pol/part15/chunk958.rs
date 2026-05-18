//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 958/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk958<F: Float>(t154: F, t506: F, t7322: F, t7326: F, t7315: F, t8589: F, t30226: F, t30240: F, t30268: F, t8775: F, t30105: F, t8952: F) -> (F, F, F, F, F, F) {
    let t33960 = t7322 * t154 * t506 * t7326;
    let t33962 = t7315 * t8589;
    let t33968 = F::new(0.17149607247227894789e-2) * t30226;
    let t33970 = F::new(0.21437009059034868486e-3) * t30240;
    let t33982 = t30268 * t8775;
    let t33984 = t30105 * t8952;
    (t33960, t33962, t33968, t33970, t33982, t33984)
}
