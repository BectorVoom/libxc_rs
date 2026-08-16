//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 757/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk757(t832: f64, t325: f64, t1347: f64, t349: f64, t854: f64, t2321: f64, t607: f64, t2288: f64, t6007: f64, t2054: f64, t761: f64, t6044: f64, t758: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6691 = t832 * t832;
    let t6692 = 1.0_f64 / t6691;
    let t6693 = t325 * t6692;
    let t6755 = 1.0_f64 / t1347 / t349;
    let t6767 = 1.0_f64 / t1347 / t854;
    let t6798 = t2321 * t607;
    let t6804 = t2288 * t6007;
    let t6806 = t2054 * t761;
    let t6809 = t758 * t6044;
    (t6691, t6692, t6693, t6755, t6767, t6798, t6804, t6806, t6809)
}
