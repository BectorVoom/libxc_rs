//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1015/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1015(t1022: f64, t9409: f64, t2861: f64, t4774: f64, t4549: f64, t9429: f64, t4802: f64, t4820: f64, t4825: f64, t10338: f64, t1754: f64, t2943: f64, t304: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14072 = t9409 * t1022;
    let t14078 = t2861 * t4774;
    let t14079 = 0.33163888888888888888e-2_f64 * t14078;
    let t14081 = t9429 * t4549;
    let t14085 = t9429 * t4802;
    let t14086 = 0.22109259259259259258e-2_f64 * t14085;
    let t14102 = t2861 * t4820;
    let t14103 = 0.66327777777777777776e-2_f64 * t14102;
    let t14104 = t2861 * t4825;
    let t14115 = t10338 * t1754;
    let t14117 = t304 * t2943;
    (t14072, t14078, t14079, t14081, t14085, t14086, t14102, t14103, t14104, t14115, t14117)
}
