//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 386/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk386(t1088: f64, t1090: f64, t123: f64, t1087: f64) -> (f64, f64, f64) {
    let t1091 = t1088 * t1090;
    let t1092 = t123 * t1091;
    let t1094 = -t1087 + 0.17808333333333333333e-1_f64 * t1092;
    (t1091, t1092, t1094)
}
