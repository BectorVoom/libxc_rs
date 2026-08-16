//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 895/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk895<F: Float>(t2605: F, t5100: F, t1604: F, t8071: F, t6086: F, t7624: F, t2147: F, t2252: F, t910: F, t551: F, t552: F, t1591: F, t2666: F) -> (F, F, F, F, F) {
    let t8227 = t5100 * t2605;
    let t8231 = F::cast_from(0.54878743191129263322e-2_f64) * t1604 * t8071;
    let t8232 = t6086 * t7624;
    let t8234 = F::cast_from(0.11643651550782197811e-1_f64) * t2147 * t8232;
    let t8235 = t910 * t2252;
    let t8237 = t551 * t552 * t8235;
    let t8240 = t1591 * t2666;
    (t8227, t8231, t8234, t8237, t8240)
}
