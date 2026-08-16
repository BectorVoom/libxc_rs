//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2532/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2532(t43816: f64, t43942: f64, t50919: f64, t51707: f64, t63361: f64, t63382: f64, t63384: f64, t63398: f64, t63400: f64, t71166: f64, t71170: f64, t71174: f64, t71179: f64, t71183: f64, t71187: f64, t71191: f64, t71195: f64, t71199: f64, t71203: f64, t71206: f64) -> f64 {
    let t71308 = -0.27469135802469135803e-1_f64 * t71166 + 0.166875e0_f64 * t71170 + 0.2225e0_f64 * t71174 + 0.18541666666666666667e-1_f64 * t71179 - 0.18541666666666666666e-1_f64 * t71183 - 0.18541666666666666666e-1_f64 * t71187 + 0.55625000000000000001e-1_f64 * t71191 - 0.11125e0_f64 * t71195 - 0.22249999999999999999e0_f64 * t71199 + 0.55625000000000000001e-1_f64 * t71203 + 0.166875e0_f64 * t71206 - 0.82407407407407407407e-2_f64 * t50919 + t51707 + t43942 - 0.96141975308641975307e-2_f64 * t43816 + 0.24722222222222222223e-1_f64 * t63361 + 0.12361111111111111111e-1_f64 * t63382 + 0.37083333333333333333e-1_f64 * t63384 - 0.37083333333333333334e-1_f64 * t63398 - 0.55625000000000000001e-1_f64 * t63400;
    t71308
}
