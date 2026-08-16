//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 422/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk422(t1878: f64, t218: f64, t220: f64, t675: f64, t679: f64, t662: f64) -> (f64, f64, f64, f64) {
    let t1880 = t218 * t1878 * t220;
    let t1881 = 0.13692777777777777778e0_f64 * t1880;
    let t1883 = t218 * t675 * t679;
    let t1897 = t662 * t662;
    (t1880, t1881, t1883, t1897)
}
