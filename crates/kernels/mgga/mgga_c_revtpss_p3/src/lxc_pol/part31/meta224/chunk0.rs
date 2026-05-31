//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1001/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1001<F: Float>(t45: F, t57: F, t4397: F, t2375: F, t5819: F, t5825: F, t78: F, t2382: F, t81: F, t162: F, t187: F, t150: F, t190: F, t1522: F, t4311: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t5927 = F::cast_from(2.0_f64) * t4397;
    let t5933 = piecewise3::<F>(t151, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2375 * t5819 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t78 * t5825);
    let t5939 = piecewise3::<F>(t155, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2382 * t5819 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t81 * t5825);
    let t5940 = t5933 + t5939;
    let t5941 = t5940 * t162;
    let t5943 = F::cast_from(0.19751673498613801407e-1_f64) * t5941 * t187;
    let t5944 = t150 * t5940;
    let t5945 = t5944 * t190;
    let t5947 = F::cast_from(8.0_f64) * t4311 * t1522;
    (t5927, t5940, t5941, t5943, t5944, t5945, t5947)
}
