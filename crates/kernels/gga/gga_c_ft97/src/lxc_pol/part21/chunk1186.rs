//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1186/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1186<F: Float>(t116254: F, t116258: F, t116262: F, t116266: F, t116270: F, t116275: F, t116279: F, t116283: F, t116287: F, t116291: F, t116294: F, t116299: F, t116302: F, t116305: F, t116310: F, t116314: F, t116318: F, t116322: F, t116326: F, t116330: F, t116334: F, t92140: F, t92143: F) -> (F, F) {
    let t117054 = t116254 / 24.0 - t116258 / 9.0 - 2.0 / 9.0 * t116262 - t116266 / 18.0 - t116270 / 18.0 + t116275 / 18.0 - t116279 / 18.0 - t116283 / 9.0 + 2.0 / 9.0 * t116287 + t116291 / 3.0 - 4.0 / 9.0 * t116294;
    let t117067 = -t116299 / 3.0 - t116302 / 4.0 + 4.0 / 3.0 * t116305 + t116310 / 18.0 + t92140 / 27.0 + 4.0 / 27.0 * t92143 + 5.0 / 81.0 * t116314 + 4.0 / 27.0 * t116318 - 2.0 / 9.0 * t116322 - 2.0 / 9.0 * t116326 - 4.0 / 9.0 * t116330 - t116334 / 18.0;
    (t117054, t117067)
}
