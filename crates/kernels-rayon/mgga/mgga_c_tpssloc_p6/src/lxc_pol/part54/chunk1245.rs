//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1245/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1245(t2039: f64, t8103: f64, t2096: f64, t33605: f64, t33611: f64, t33615: f64, t33619: f64, t33622: f64, t33624: f64, t33746: f64, t4028: f64, t652: f64, t7266: f64, t7458: f64, t7802: f64, t7806: f64, t7904: f64, t7941: f64, t8690: f64, t8835: f64) -> (f64, f64) {
    let t34170 = t8103 * t2039;
    let t34173 = t2096 * t33746 - 2.0_f64 * t34170 * t652 - 2.0_f64 * t4028 * t8835 - 2.0_f64 * t7266 * t7802 - 2.0_f64 * t7266 * t7806 - 2.0_f64 * t7458 * t8835 + 3.0_f64 * t7904 * t8690 + t7941 * t8690 + t33605 - t33611 + t33615 - t33619 - t33622 - t33624;
    (t34170, t34173)
}
