//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1147/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1147<F: Float>(t5229: F, t943: F, t1108: F, t938: F, t15286: F, t4223: F, t15281: F, t4219: F, t12399: F, t14906: F, t3931: F, t1098: F, t1116: F, t1125: F, t12439: F, t12443: F, t4212: F, t4220: F, t4224: F, t9658: F, t9669: F, t9701: F) -> F {
    let t15805 = t5229 * t943;
    let t15807 = t938 * t1108 * t15805;
    let t15814 = t4223 * t15286;
    let t15819 = t4219 * t15281;
    let t15822 = t12399 * t14906;
    let t15823 = t3931 * t15822;
    let t15826 = F::cast_from(19.0_f64) / F::cast_from(1728.0_f64) * t15807 * t1116 + t9658 / F::cast_from(1296.0_f64) + t9669 / F::cast_from(20736.0_f64) + t4212 * t4224 / F::cast_from(27.0_f64) - t1098 * t15814 / F::cast_from(144.0_f64) - F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t4212 * t4220 + t1098 * t15819 / F::cast_from(216.0_f64) + t12439 + t12443 + t9701 - t1125 * t15823 / F::cast_from(768.0_f64);
    t15826
}
