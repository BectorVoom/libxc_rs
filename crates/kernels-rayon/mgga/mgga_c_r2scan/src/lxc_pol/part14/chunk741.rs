//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 741/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk741(t166: f64, t6006: f64, t6007: f64, t2055: f64, t2056: f64, t607: f64, t2050: f64, t761: f64, t2054: f64, t58: f64, t423: f64, t597: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6010 = 0.1714584e0_f64 * t6006 * t166 * t6007;
    let t6012 = t2055 * t607 * t2056;
    let t6026 = 0.1714584e0_f64 * t2055 * t2050 * t761;
    let t6027 = t2054 * t58;
    let t6028 = t6027 * t423;
    let t6029 = t597 * t2056;
    (t6010, t6012, t6026, t6027, t6028, t6029)
}
