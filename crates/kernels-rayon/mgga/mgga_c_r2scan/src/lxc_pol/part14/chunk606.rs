//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 606/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk606(t254: f64, t3344: f64, t2333: f64, t795: f64, t321: f64, t502: f64, t263: f64, t818: f64) -> (f64, f64, f64, f64) {
    let t3345 = t254 * t3344;
    let t3352 = t2333 * t795;
    let t3356 = t502 * t321;
    let t3358 = t263 * t818;
    (t3345, t3352, t3356, t3358)
}
