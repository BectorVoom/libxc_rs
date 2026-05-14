//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 832/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk832<F: Float>(t45: F, t57: F, t2382: F, t5819: F, t5825: F, t81: F, t5933: F, t162: F, t187: F, t150: F, t190: F, t1522: F, t4311: F, t4399: F, t766: F, t80: F, t770: F, t83: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t5939 = piecewise3(t155, 0.0, 4.0 / 9.0 * t2382 * t5819 - 4.0 / 3.0 * t81 * t5825);
    let t5940 = t5933 + t5939;
    let t5941 = t5940 * t162;
    let t5943 = 0.19751673498613801407e-1 * t5941 * t187;
    let t5944 = t150 * t5940;
    let t5945 = t5944 * t190;
    let t5947 = 8.0 * t4311 * t1522;
    let t5948 = 0.11696447245269292414e1 * t4399;
    let t5954 = piecewise3(t151, 0.0, -2.0 / 9.0 * t80 * t5819 + 2.0 / 3.0 * t766 * t5825);
    let t5960 = piecewise3(t155, 0.0, -2.0 / 9.0 * t83 * t5819 - 2.0 / 3.0 * t770 * t5825);
    (t5940, t5941, t5943, t5944, t5945, t5947, t5948, t5954, t5960)
}
