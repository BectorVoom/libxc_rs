//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 953/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk953(t10899: f64, t3320: f64, t783: f64, t774: f64, t787: f64) -> (f64, f64) {
    let t10901 = t783 * t10899 * t3320;
    let t10902 = 0.22511059664845582436e0_f64 * t10901;
    let t10903 = t774 * t787;
    (t10902, t10903)
}
