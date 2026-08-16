//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1370/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1370(t102398: f64, t102421: f64, t103251: f64, t103263: f64, t103328: f64, t103423: f64, t103496: f64, t103502: f64, t103507: f64, t103525: f64, t20882: f64, t27369: f64, t27438: f64, t27453: f64, t5701: f64, t5709: f64, t7908: f64, t94227: f64, t94626: f64) -> f64 {
    let t103527 = -0.46336805555555555557e-3_f64 * t103496 + 0.66327777777777777776e-2_f64 * t102398 - 0.92673611111111111112e-3_f64 * t94626 * t103263 - 0.18550940104166666667e-3_f64 * t94227 * t103502 + 0.61782407407407407408e-3_f64 * t94626 * t103507 - 0.61836467013888888889e-4_f64 * t94227 * t103328 + 0.46336805555555555556e-3_f64 * t7908 * t5709 * t27453 * t20882 + 0.30918233506944444445e-4_f64 * t27369 * t103251 - 0.30891203703703703704e-3_f64 * t7908 * t5701 * t27438 * t20882 + 0.14739506172839506172e-2_f64 * t102421 + 0.23168402777777777778e-3_f64 * t7908 * t103423 - 0.22653549382716049382e-2_f64 * t103525;
    t103527
}
