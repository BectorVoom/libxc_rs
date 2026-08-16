//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1419/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1419(t1661: f64, t71445: f64, t71448: f64, t18754: f64, t5999: f64, t18746: f64, t43895: f64, t63361: f64, t78057: f64, t78084: f64, t78087: f64, t78090: f64, t78093: f64, t78095: f64, t78097: f64, t78100: f64) -> (f64, f64, f64, f64, f64) {
    let t78103 = t71445 * t1661;
    let t78105 = t71448 * t1661;
    let t78107 = t18754 * t5999;
    let t78109 = t18746 * t5999;
    let t78112 = -0.11038e0_f64 * t78084 - 0.99342e0_f64 * t78087 + 0.66228e0_f64 * t78090 + 0.298026e1_f64 * t78093 + 0.258925e1_f64 * t78095 + t43895 + 0.247573125e0_f64 * t78097 + 0.22076e0_f64 * t78100 + 0.16102666666666666667e1_f64 * t63361 + 0.3300975e0_f64 * t78103 - 0.51785e1_f64 * t78105 + 0.11651625e2_f64 * t78107 - 0.247573125e0_f64 * t78109 - 0.72462e1_f64 * t78057;
    (t78103, t78105, t78107, t78109, t78112)
}
