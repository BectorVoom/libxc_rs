//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1157/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1157(t10491: f64, t5039: f64, t3325: f64, t5189: f64, t1820: f64, t3331: f64, t10498: f64, t1203: f64, t3330: f64, t3481: f64, t13260: f64, t5181: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14674 = 4.0_f64 * t10491 * t5039;
    let t14676 = 2.0_f64 * t3325 * t5189;
    let t14677 = t1820 * t3331;
    let t14679 = 6.0_f64 * t10498 * t14677;
    let t14680 = t5189 * t1203;
    let t14682 = 4.0_f64 * t3330 * t14680;
    let t14683 = t1820 * t3481;
    let t14685 = 2.0_f64 * t3330 * t14683;
    let t14686 = t5181 * t13260;
    (t14674, t14676, t14679, t14682, t14685, t14686)
}
