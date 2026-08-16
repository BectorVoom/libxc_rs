//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 191/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk191(t719: f64, t735: f64, t734: f64, t704: f64, t716: f64, sigma2: f64) -> (f64, f64, f64, f64) {
    let t736 = t735 * t719;
    let t737 = t734 * t736;
    let t739 = t704 * t716;
    let t740 = sigma2 * sigma2;
    (t736, t737, t739, t740)
}
