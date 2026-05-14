//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1206/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1206<F: Float>(t22332: F, t6012: F, t20934: F, t4293: F, t4292: F, t17470: F, t21910: F, t5903: F, t1497: F, t6917: F, t5909: F, t4260: F, t2035: F, t6041: F, t22252: F, t6011: F) -> (F, F, F, F, F, F, F) {
    let t22333 = t22332 * t6012;
    let t22335 = t4293 * t20934;
    let t22336 = t4292 * t22335;
    let t22338 = t17470 * t21910;
    let t22339 = t5903 * t22338;
    let t22341 = t6917 * t1497;
    let t22342 = t5909 * t22341;
    let t22343 = t4260 * t22342;
    let t22345 = t2035 * t6041;
    let t22348 = t6011 * t22252;
    (t22333, t22336, t22339, t22341, t22343, t22345, t22348)
}
