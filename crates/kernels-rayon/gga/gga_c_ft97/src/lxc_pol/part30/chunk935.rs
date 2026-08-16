//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 935/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk935(t1434: f64, t2399: f64, t7524: f64, t2347: f64, t7440: f64, t33453: f64, t681: f64, t7484: f64, t1882: f64, t33462: f64, t33478: f64, t7514: f64, t898: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t140768 = t1434 * t2399 * t7524;
    let t140769 = 2.0_f64 / 9.0_f64 * t140768;
    let t140774 = t7440 * t2347;
    let t140784 = t1434 * t681 * t33453;
    let t140790 = t7484 * t2347;
    let t140795 = t1882 * t33462;
    let t140797 = t1882 * t33478;
    let t140833 = t898 * t7514;
    (t140768, t140769, t140774, t140784, t140790, t140795, t140797, t140833)
}
