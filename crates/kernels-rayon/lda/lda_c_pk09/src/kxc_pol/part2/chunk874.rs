//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 874/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk874(t3323: f64, t3326: f64, t3424: f64, t3426: f64, t3428: f64, t4313: f64, t4320: f64, t4322: f64, t7870: f64, t7875: f64, t7879: f64, t7884: f64, t7888: f64) -> f64 {
    let t9109 = 0.10188339589005964_f64 * t3323 + 0.10188339589005964_f64 * t3326 + t4313 + 4.596908415362055_f64 * t7870 - 4.596908415362055_f64 * t7875 + 4.596908415362055_f64 * t7879 - 4.596908415362055_f64 * t7884 + 4.596908415362055_f64 * t7888 + 3.06460561024137_f64 * t3424 + 3.06460561024137_f64 * t3426 - 3.06460561024137_f64 * t3428 + t4320 + t4322;
    t9109
}
