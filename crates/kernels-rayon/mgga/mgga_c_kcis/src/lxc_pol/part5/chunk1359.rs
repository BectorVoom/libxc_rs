//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1359/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1359(t21905: f64, t5904: f64, t4292: f64, t20934: f64, t4261: f64, t4260: f64, t21804: f64, t4293: f64, t6010: f64, t2034: f64, t492: f64, t5910: f64) -> (f64, f64, f64, f64) {
    let t22381 = t5904 * t21905;
    let t22382 = t4292 * t22381;
    let t22384 = t4261 * t20934;
    let t22385 = t4260 * t22384;
    let t22387 = t4293 * t21804;
    let t22388 = t6010 * t22387;
    let t22390 = t2034 * t492;
    let t22391 = t22390 * t5910;
    (t22382, t22385, t22388, t22391)
}
