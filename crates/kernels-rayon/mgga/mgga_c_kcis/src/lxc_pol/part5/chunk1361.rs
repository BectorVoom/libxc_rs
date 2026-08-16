//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1361/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1361(t22403: f64, t4260: f64, t21905: f64, t5909: f64, t21038: f64, t5908: f64, t12530: f64, t7299: f64, t12575: f64, t7318: f64, t12568: f64, t7338: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22404 = t4260 * t22403;
    let t22406 = t5909 * t21905;
    let t22407 = t4260 * t22406;
    let t22410 = t5909 * t21038;
    let t22411 = t5908 * t22410;
    let t22413 = t12530 * t7299;
    let t22415 = t12575 * t7318;
    let t22417 = t12568 * t7338;
    (t22404, t22407, t22411, t22413, t22415, t22417)
}
