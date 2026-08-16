//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1866/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1866(t20760: f64, t20761: f64, t20765: f64, t20766: f64, t20768: f64, t9724: f64, t9780: f64, t9789: f64, t9793: f64, t9797: f64, t9863: f64, t4205: f64, t5597: f64) -> (f64, f64) {
    let t20812 = t9724 + t9863 + t9780 - t20760 + t20761 + t20765 + t20766 + t20768 - t9789 + t9793 + t9797;
    let t20815 = 12.0_f64 * t4205 * t5597;
    (t20812, t20815)
}
