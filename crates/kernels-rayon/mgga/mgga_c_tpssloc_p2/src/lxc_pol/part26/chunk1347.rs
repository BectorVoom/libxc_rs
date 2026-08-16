//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1347/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1347(t7291: f64, t85660: f64, t24564: f64, t24574: f64, t1090: f64, t11504: f64, t1186: f64, t1251: f64, t2123: f64, t2250: f64, t24589: f64, t24590: f64, t24601: f64, t24602: f64, t24611: f64, t24880: f64, t24887: f64, t27549: f64, t3631: f64, t7283: f64, t7287: f64, t85628: f64, t85640: f64, t85642: f64, t85643: f64, t85648: f64, t85652: f64) -> f64 {
    let t85661 = t85660 * t7291;
    let t85669 = t24574 * t24564;
    let t85673 = 0.82246703342411321826e-2_f64 * t24589 * t24601 * t85628 * t1090 + 0.82246703342411321826e-2_f64 * t24589 * t24601 * t24602 * t2250 * t1251 + 0.54831135561607547883e-2_f64 * t85640 - 0.10966227112321509577e-1_f64 * t27549 * t24601 * t85642 * t85643 + 0.82246703342411321826e-2_f64 * t24589 * t85648 * t7287 + 0.16449340668482264365e-1_f64 * t24589 * t24601 * t85652 * t85643 + 0.16449340668482264365e-1_f64 * t24589 * t24590 * t24887 + 0.54831135561607547884e-2_f64 * t85661 - 0.24674011002723396548e-1_f64 * t7283 * t1186 * t24611 - 0.82246703342411321825e-2_f64 * t7283 * t11504 * t2123 - 0.82246703342411321826e-2_f64 * t85669 - 3.0_f64 * t24880 * t3631;
    t85673
}
