//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 447/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk447(t809: f64, t87: f64, t820: f64, t98: f64, t1685: f64, t68: f64, t131: f64, t117: f64, t504: f64, t325: f64, t623: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4861 = t87 * t809;
    let t4882 = t98 * t820;
    let t4961 = t68 * t1685;
    let t4962 = t4961 * t131;
    let t4965 = t504 * t117;
    let t4985 = t623 * t325;
    (t4861, t4882, t4961, t4962, t4965, t4985)
}
