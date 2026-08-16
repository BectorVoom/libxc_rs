//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 573/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk573(t3270: f64, t769: f64, t314: f64, t9688: f64, t313: f64, t3276: f64, t3266: f64, t10031: f64, t10033: f64, t10037: f64, t10042: f64, t10043: f64, t2028: f64, t2098: f64, t2194: f64, t2197: f64, t317: f64, t3267: f64, t3291: f64, t3300: f64, t6119: f64, t784: f64, t797: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10050 = t769 * t3270;
    let t10053 = t314 * t9688;
    let t10054 = t313 * t10053;
    let t10057 = t769 * t3276;
    let t10062 = t769 * t3266;
    let t10065 = -t10031 - 0.39722766613167140743e-1_f64 * t10033 * t2028 + 0.11916829983950142223e0_f64 * t10037 * t6119 + t10042 - 0.35750489951850426669e0_f64 * t797 * t10043 - 0.46011511144704899612e1_f64 * t2194 * t3291 + 0.11502877786176224903e2_f64 * t2197 * t3300 + 0.35750489951850426669e0_f64 * t10050 * t317 + 0.35750489951850426669e0_f64 * t10054 * t317 - 0.10725146985555128001e1_f64 * t10057 * t2098 + 0.23833659967900284446e0_f64 * t3267 * t784 + 0.35750489951850426669e0_f64 * t10062 * t317;
    (t10050, t10053, t10054, t10057, t10062, t10065)
}
