//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 841/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk841(t2210: f64, t3273: f64, t3826: f64, t3829: f64, t4528: f64, t4530: f64, t4531: f64, t7598: f64, t7602: f64, t8577: f64, t8585: f64, t8587: f64, t8589: f64, t8592: f64, t8595: f64, t8597: f64) -> f64 {
    let t8599 = -t4528 + t4530 + 0.8357942709722364_f64 * t8577 + 38.978347549160304_f64 * t3826 * t7598 + 19.489173774580152_f64 * t3826 * t7602 - 19.489173774580152_f64 * t3829 * t2210 + 19.489173774580152_f64 * t8585 - 12.992782516386768_f64 * t8587 + 2.507382812916709_f64 * t8589 - 1.2536914064583544_f64 * t4531 + 2.0_f64 * t3273 * t8592 + 3.600163427964126_f64 * t8595 + 3.600163427964126_f64 * t8597;
    t8599
}
