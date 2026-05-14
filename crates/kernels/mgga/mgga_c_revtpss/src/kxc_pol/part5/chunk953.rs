//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 953/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk953<F: Float>(t11298: F, t275: F, t11132: F, t240: F, t624: F, t281: F, t283: F, t3252: F, t276: F, t285: F, t273: F, t2439: F, t931: F, t2922: F, t913: F, t290: F, t2925: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11299 = t275 * t11298;
    let t11304 = 28.0 / 27.0 * t11132;
    let t11334 = 0.93011851851851851854e0 * t11132;
    let t11335 = t624 * t240;
    let t11337 = t281 * t11335 * t283;
    let t11338 = 0.36514074074074074075e0 * t11337;
    let t11341 = t240 * t3252;
    let t11354 = 1.0 / t276 / t285 / 4.0;
    let t11358 = 1.0/pow_3_2(t273);
    let t11366 = t2439 * t931;
    let t11384 = 1.0 / t2922 / t913;
    let t11385 = t275 * t11384;
    let t11387 = 1.0 / t2925 / t290;
    (t11299, t11304, t11334, t11335, t11337, t11338, t11341, t11354, t11358, t11366, t11385, t11387)
}
