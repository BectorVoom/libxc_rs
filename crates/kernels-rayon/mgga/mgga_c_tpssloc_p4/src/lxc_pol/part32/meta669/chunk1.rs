//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2102/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2102(t27495: f64, t27497: f64, t95195: f64, t1170: f64, t2121: f64, t27732: f64, t15590: f64, t7338: f64, t27614: f64, t3572: f64, t27617: f64, t3523: f64) -> (f64, f64, f64, f64, f64) {
    let t95201 = t95195 * t27495 * t27497;
    let t95213 = 0.54831135561607547884e-2_f64 * t2121 * t1170 * t27732;
    let t95238 = t15590 * t7338;
    let t95242 = t27614 * t3572 / 1152.0_f64;
    let t95244 = t27617 * t3523 / 1728.0_f64;
    (t95201, t95213, t95238, t95242, t95244)
}
