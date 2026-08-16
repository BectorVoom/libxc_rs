//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 123/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk123(t417: f64, t429: f64, t430: f64, t431: f64, t435: f64, t436: f64, t437: f64, t443: f64, t445: f64, t446: f64, t447: f64) -> f64 {
    let t451 = -0.11955719325063177623e-1_f64 * t417 + 0.263475e-2_f64 * t429 * t430 * t431 - 0.4755e-3_f64 * t435 * t436 * t437 + 0.2589769453898153438e-4_f64 * t443 - 0.21605625e-5_f64 * t445 * t446 * t447;
    t451
}
