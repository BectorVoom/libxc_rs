//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1363/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1363(t756: f64, t9874: f64, t9727: f64, t9780: f64, t9789: f64, t9793: f64, t9797: f64, t9863: f64, t9865: f64, t9867: f64, t9870: f64, t9872: f64) -> (f64, f64) {
    let t9876 = 0.56968947174242584612e-3_f64 * t756 * t9874;
    let t9877 = t9727 + t9863 + t9780 + t9865 - t9867 - t9789 + t9870 + t9872 + t9793 + t9797 - t9876;
    (t9876, t9877)
}
