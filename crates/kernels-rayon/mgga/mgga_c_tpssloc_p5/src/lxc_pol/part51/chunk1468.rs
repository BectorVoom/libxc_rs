//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1468/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1468(t120800: f64, t120803: f64, t122776: f64, t122780: f64, t122784: f64, t122786: f64, t122788: f64, t122790: f64, t122794: f64, t2039: f64, t23877: f64, t31795: f64, t4072: f64, t7801: f64, t7956: f64, t83980: f64, t86656: f64) -> f64 {
    let t122797 = t122776 + 0.135e2_f64 * t31795 * t4072 + t122780 + 27.0_f64 * t83980 * t7956 + t122784 + t122786 + t122788 + t122790 + 0.135e2_f64 * t23877 * t7801 + t120800 + t120803 + t122794 + 0.135e2_f64 * t86656 * t2039;
    t122797
}
