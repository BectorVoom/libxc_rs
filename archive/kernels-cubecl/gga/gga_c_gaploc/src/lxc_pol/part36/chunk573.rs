//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 573/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk573<F: Float>(t3270: F, t769: F, t314: F, t9688: F, t313: F, t3276: F, t3266: F, t10031: F, t10033: F, t10037: F, t10042: F, t10043: F, t2028: F, t2098: F, t2194: F, t2197: F, t317: F, t3267: F, t3291: F, t3300: F, t6119: F, t784: F, t797: F) -> (F, F, F, F, F, F) {
    let t10050 = t769 * t3270;
    let t10053 = t314 * t9688;
    let t10054 = t313 * t10053;
    let t10057 = t769 * t3276;
    let t10062 = t769 * t3266;
    let t10065 = -t10031 - F::cast_from(0.39722766613167140743e-1_f64) * t10033 * t2028 + F::cast_from(0.11916829983950142223e0_f64) * t10037 * t6119 + t10042 - F::cast_from(0.35750489951850426669e0_f64) * t797 * t10043 - F::cast_from(0.46011511144704899612e1_f64) * t2194 * t3291 + F::cast_from(0.11502877786176224903e2_f64) * t2197 * t3300 + F::cast_from(0.35750489951850426669e0_f64) * t10050 * t317 + F::cast_from(0.35750489951850426669e0_f64) * t10054 * t317 - F::cast_from(0.10725146985555128001e1_f64) * t10057 * t2098 + F::cast_from(0.23833659967900284446e0_f64) * t3267 * t784 + F::cast_from(0.35750489951850426669e0_f64) * t10062 * t317;
    (t10050, t10053, t10054, t10057, t10062, t10065)
}
