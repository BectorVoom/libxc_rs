//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1336/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1336(t12524: f64, t23893: f64, t23896: f64, t2169: f64, t3946: f64, t1404: f64, t7415: f64, t2174: f64, t3931: f64, t24954: f64, t580: f64, t111: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t84016 = 162.0_f64 * t12524 * t23893;
    let t84018 = 81.0_f64 * t12524 * t23896;
    let t85403 = t2169 * t3946;
    let t85405 = t7415 * t1404;
    let t85407 = t3931 * t2174;
    let t85412 = t24954 * t580;
    let t85416 = t7415 * t111;
    (t84016, t84018, t85403, t85405, t85407, t85412, t85416)
}
