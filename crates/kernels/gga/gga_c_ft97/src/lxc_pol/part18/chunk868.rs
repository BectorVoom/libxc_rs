//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 868/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk868<F: Float>(t86: F, t22913: F, t23387: F, t113: F, t1342: F, t1934: F, t5: F, t505: F, t5756: F, t1359: F, t7368: F, t165: F, t1986: F, t28: F, t1348: F, t458: F) -> (F, F, F, F, F, F, F) {
    let t87 = 10000000.0 <= t86;
    let t23388 = t22913 + t23387;
    let t23399 = piecewise3(t87, 0.0, t5 * t23388 * t113 / 4.0 + t5 * t5756 * t505 / 2.0 + t5 * t1342 * t1934 / 4.0);
    let t23400 = t7368 * t1359;
    let t23401 = t165 * t1986;
    let t23402 = t23400 * t23401;
    let t23403 = t28 * t23402;
    let t23405 = t1348 * t458;
    (t23388, t23399, t23400, t23401, t23402, t23403, t23405)
}
