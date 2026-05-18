//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 303/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk303<F: Float>(t1218: F, t1249: F, t1253: F, t1255: F, t301: F, t317: F, t332: F, t231: F, t893: F, t992: F, t1093: F, t1190: F, t902: F) -> (F, F, F, F) {
    let t1258 = -t1218 * t317 - t1253 * t301 - F::new(2.0) * t1249 + F::new(2.0) * t1255;
    let t1259 = t1258 * t332;
    let t1263 = t231 * t893 * t992;
    let t1268 = F::new(0.234754e0) * t1190 - t902 - F::new(0.14443083333333333333e0) * t1093;
    (t1258, t1259, t1263, t1268)
}
