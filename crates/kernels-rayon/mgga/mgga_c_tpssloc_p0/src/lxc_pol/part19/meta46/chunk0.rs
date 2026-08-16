//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 308/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk308(t893: f64, t914: f64, t880: f64, t886: f64) -> (f64, f64) {
    let t916 = 1.0_f64 * t893 * t914;
    let t917 = 0.17123333333333333333e-1_f64 * t880;
    let t919 = -t917 - 0.17123333333333333333e-1_f64 * t886;
    (t916, t919)
}
