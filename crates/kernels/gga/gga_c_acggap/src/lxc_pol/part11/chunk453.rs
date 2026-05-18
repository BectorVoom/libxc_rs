//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 453/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk453<F: Float>(t2118: F, t409: F, t2042: F, t2044: F, t2049: F, t2053: F, t2057: F, t2063: F, t2071: F, t2075: F, t2078: F, t2084: F, t2088: F, t2093: F, t2098: F, t2102: F, t2105: F, t2110: F, t2114: F) -> F {
    let t2119 = t2118 * t409;
    let t2121 = -t2042 / F::new(48.0) - t2044 / F::new(96.0) - t2049 / F::new(128.0) - t2053 / F::new(384.0) - F::new(0.38203125e-2) * t2057 + F::new(0.7640625e-2) * t2063 - F::new(0.10718504529517434243e-3) * t2071 + F::new(0.15724046144802076034e-3) * t2075 + t2078 - t2084 - F::new(0.53592522647587171215e-3) * t2088 - F::new(0.21437009059034868486e-3) * t2093 - t2098 + t2102 + F::new(0.7862023072401038017e-3) * t2105 - F::new(0.31448092289604152068e-3) * t2110 + F::new(0.47172138434406228102e-3) * t2114 - F::new(0.42874018118069736972e-3) * t2119;
    t2121
}
