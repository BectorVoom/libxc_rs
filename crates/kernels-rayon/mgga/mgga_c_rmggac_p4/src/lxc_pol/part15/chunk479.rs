//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 479/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk479(t5982: f64, t5995: f64, t6005: f64, t6015: f64, t1847: f64, t453: f64, t1839: f64, t446: f64, t1392: f64, t589: f64, t1835: f64, t1794: f64, t4396: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6017 = t5982 + t5995 + t6005 + t6015;
    let t6020 = t1847 * t453;
    let t6031 = t1839 * t446;
    let t6034 = t589 * t1392;
    let t6039 = t1835 * t446;
    let t6042 = t4396 * t1794;
    (t6017, t6020, t6031, t6034, t6039, t6042)
}
