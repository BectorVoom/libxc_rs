//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2495/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2495(t1003: f64, t1058: f64, t1060: f64, t11046: f64, t11048: f64, t14618: f64, t14651: f64, t18099: f64, t18121: f64, t18155: f64, t21615: f64, t21622: f64, t21626: f64, t3200: f64, t4615: f64, t4657: f64, t4669: f64, t4684: f64, t4691: f64, t50592: f64, t5866: f64, t5903: f64, t5937: f64, t5939: f64, t5941: f64, t70014: f64) -> f64 {
    let t70970 = 3.0_f64 * t1058 * t1060 * t4657 * t5866 + t11046 * t11048 * t70014 - 3.0_f64 * t18099 * t21622 * t3200 - 3.0_f64 * t21626 * t3200 * t4684 + t1003 * t21615 + 6.0_f64 * t14618 * t18121 + 3.0_f64 * t14651 * t5937 + 3.0_f64 * t18155 * t4669 + 3.0_f64 * t4615 * t5941 + 3.0_f64 * t4691 * t5903 - 3.0_f64 * t50592 * t5939;
    t70970
}
