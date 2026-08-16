//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 453/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk453(t2118: f64, t409: f64, t2042: f64, t2044: f64, t2049: f64, t2053: f64, t2057: f64, t2063: f64, t2071: f64, t2075: f64, t2078: f64, t2084: f64, t2088: f64, t2093: f64, t2098: f64, t2102: f64, t2105: f64, t2110: f64, t2114: f64) -> f64 {
    let t2119 = t2118 * t409;
    let t2121 = -t2042 / 48.0_f64 - t2044 / 96.0_f64 - t2049 / 128.0_f64 - t2053 / 384.0_f64 - 0.38203125e-2_f64 * t2057 + 0.7640625e-2_f64 * t2063 - 0.10718504529517434243e-3_f64 * t2071 + 0.15724046144802076034e-3_f64 * t2075 + t2078 - t2084 - 0.53592522647587171215e-3_f64 * t2088 - 0.21437009059034868486e-3_f64 * t2093 - t2098 + t2102 + 0.7862023072401038017e-3_f64 * t2105 - 0.31448092289604152068e-3_f64 * t2110 + 0.47172138434406228102e-3_f64 * t2114 - 0.42874018118069736972e-3_f64 * t2119;
    t2121
}
