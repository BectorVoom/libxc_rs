//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 277/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk277(t207: f64, t792: f64, t795: f64, t785: f64, t787: f64, t789: f64) -> (f64, f64) {
    let t797 = 0.41666666666666666666e-3_f64 * t792 * t207 * t795;
    let t798 = -t785 - 0.16666666666666666666e-2_f64 * t787 * t789 - t797;
    (t797, t798)
}
