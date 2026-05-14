//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 754/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk754<F: Float>(t1331: F, t2331: F, t251: F, t3977: F, t11407: F, t250: F, t3106: F, t461: F, t453: F, t1314: F, t3897: F, t455: F, t3900: F, t468: F, t1346: F, t3943: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11455 = t2331 * t1331;
    let t11462 = t251 * t3977;
    let t11479 = 0.93932222222222222223e0 * t11407;
    let t11481 = t250 * t3106 * t461;
    let t11482 = 0.36793333333333333333e0 * t11481;
    let t11491 = 1.0/pow_3_2(t453);
    let t11512 = 1.0 / t3897 / t1314;
    let t11513 = t455 * t11512;
    let t11516 = 1.0 / t3900 / t468;
    let t11520 = 0.28842592592592592592e-1 * t11407;
    let t11536 = 1.0 / t3943 / t1346;
    (t11455, t11462, t11479, t11481, t11482, t11491, t11513, t11516, t11520, t11536)
}
