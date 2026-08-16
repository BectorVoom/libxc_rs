//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 748/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk748(t204: f64, t334: f64, t3981: f64, t1281: f64, t824: f64) -> (f64, f64, f64) {
    let t6087 = t204 * t3981 * t334;
    let t6088 = 0.55403703703703703703e-1_f64 * t6087;
    let t6090 = t204 * t1281 * t824;
    (t6087, t6088, t6090)
}
