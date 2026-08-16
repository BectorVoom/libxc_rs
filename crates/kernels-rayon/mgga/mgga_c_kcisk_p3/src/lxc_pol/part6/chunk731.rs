//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 731/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk731(t344: f64, t3118: f64, t313: f64, t353: f64, t347: f64, t355: f64, t13522: f64, t1232: f64, t4079: f64, t346: f64, t360: f64, t4082: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13632 = 1.0_f64/pow_3_2(t344);
    let t13665 = t353 * t3118 * t313;
    let t13666 = 0.73028148148148148147e0_f64 * t13665;
    let t13669 = 1.0_f64 / t347 / t355 / 8.0_f64;
    let t13672 = 0.93011851851851851854e0_f64 * t13522;
    let t13679 = 1.0_f64 / t4079 / t1232;
    let t13680 = t346 * t13679;
    let t13682 = 1.0_f64 / t4082 / t360;
    (t13632, t13665, t13666, t13669, t13672, t13680, t13682)
}
