//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 850/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk850(t155: f64, t205: f64, t2341: f64, t3758: f64, t4633: f64, t4643: f64, t4649: f64, t4652: f64, t7768: f64, t7776: f64, t8096: f64, t8101: f64, t8524: f64, t8731: f64, t8734: f64, t8744: f64, t8748: f64, t976: f64) -> f64 {
    let t8757 = 0.29617398950766044_f64 * t8731 * t8734 - 7.108175748183851_f64 * t3758 * t8524 - 19.489173774580152_f64 * t155 * t7768 - 19.489173774580152_f64 * t155 * t7776 - t4633 + 2.3693919160612835_f64 * t205 * t8744 + 2.3693919160612835_f64 * t205 * t8748 - t4643 - t4649 - t4652 - 19.489173774580152_f64 * t155 * t8096 - 19.489173774580152_f64 * t155 * t8101 + 19.489173774580152_f64 * t976 * t2341;
    t8757
}
