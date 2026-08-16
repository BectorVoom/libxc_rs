//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2562/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2562(t3264: f64, t4782: f64, t6020: f64, t1671: f64, t18834: f64, t11185: f64, t21899: f64, t1670: f64, t3313: f64, t63588: f64, t18258: f64, t4781: f64) -> (f64, f64, f64, f64, f64) {
    let t71806 = 6.0_f64 * t3264 * t4782 * t6020;
    let t71809 = 6.0_f64 * t3264 * t1671 * t18834;
    let t71811 = 0.48245938496077605201e2_f64 * t11185 * t21899;
    let t71814 = 0.48245938496077605201e2_f64 * t3313 * t63588 * t1670;
    let t71817 = 0.48245938496077605201e2_f64 * t3313 * t18258 * t4781;
    (t71806, t71809, t71811, t71814, t71817)
}
