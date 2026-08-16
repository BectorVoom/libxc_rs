//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 727/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk727(t334: f64, t371: f64, t2018: f64, t532: f64, t1984: f64, t6546: f64) -> (f64, f64, f64, f64) {
    let t6793 = t371 * t334;
    let t6794 = 1.0_f64 / t6793;
    let t6878 = t532 * t2018;
    let t6883 = t6546 * t1984;
    (t6793, t6794, t6878, t6883)
}
