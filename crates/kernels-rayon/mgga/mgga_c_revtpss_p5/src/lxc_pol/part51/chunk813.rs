//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 813/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk813(t25969: f64, t2482: f64, t27: f64, t7269: f64, t3981: f64, t2019: f64, t3985: f64, t820: f64, t843: f64, t1416: f64, t3999: f64, t64: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25970 = 0.27104001498285508387e-3_f64 * t25969;
    let t25972 = t2482 * t7269 * t27;
    let t25973 = t25972 * t3981;
    let t25975 = t2019 * t3985;
    let t25976 = 0.11337795902333997111e-1_f64 * t25975;
    let t25978 = t820 * t7269 * t843;
    let t25979 = t25978 * t1416;
    let t25981 = t3999 * t64;
    (t25970, t25972, t25973, t25976, t25978, t25979, t25981)
}
