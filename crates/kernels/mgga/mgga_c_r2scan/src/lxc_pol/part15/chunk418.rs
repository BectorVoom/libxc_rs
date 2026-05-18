//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 418/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk418<F: Float>(t1719: F, t219: F, t225: F, t61: F, t153: F, t158: F) -> (F, F, F, F, F) {
    let t1721 = t219 * t1719 * t225;
    let t1723 = F::new(0.65061487801810439052e-1) * t61 * t1721;
    let t1724 = t153 * t153;
    let t1725 = F::new(1.0) / t1724;
    let t1726 = t1725 * t158;
    (t1721, t1723, t1724, t1725, t1726)
}
