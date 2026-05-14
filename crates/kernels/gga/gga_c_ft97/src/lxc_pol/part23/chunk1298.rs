//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1298/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1298<F: Float>(t1403: F, t30861: F, t681: F, t10052: F, t109760: F, t122162: F, t122166: F, t122432: F, t122662: F, t124662: F, t124686: F, t125072: F, t1425: F, t18123: F, t18757: F, t193: F, t2354: F, t24245: F, t30860: F, t30930: F, t4973: F, t6002: F, t6003: F, t766: F, t771: F) -> (F,) {
    let t125247 = t1403 * t681 * t30861;
    let t125271 = -4.0 * t124686 - 12.0 * t10052 * t30930 * t766 - t125247 / 18.0 - t6002 * t2354 * t24245 * t4973 / 18.0 - t6002 * t2354 * t6003 * t18123 / 18.0 + 8.0 * t122662 + 8.0 * t124662 + 8.0 * t122432 - 12.0 * t122162 + 8.0 * t122166 + t1403 * t193 * t1425 * t18757 / 6.0 + 4.0 * t125072 + t1403 * t193 * t30860 * t771 / 6.0 + t109760;
    (t125271,)
}
