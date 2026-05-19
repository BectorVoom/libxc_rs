//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 841/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk841<F: Float>(t2210: F, t3273: F, t3826: F, t3829: F, t4528: F, t4530: F, t4531: F, t7598: F, t7602: F, t8577: F, t8585: F, t8587: F, t8589: F, t8592: F, t8595: F, t8597: F) -> F {
    let t8599 = -t4528 + t4530 + F::cast_from(0.8357942709722364_f64) * t8577 + F::cast_from(38.978347549160304_f64) * t3826 * t7598 + F::cast_from(19.489173774580152_f64) * t3826 * t7602 - F::cast_from(19.489173774580152_f64) * t3829 * t2210 + F::cast_from(19.489173774580152_f64) * t8585 - F::cast_from(12.992782516386768_f64) * t8587 + F::cast_from(2.507382812916709_f64) * t8589 - F::cast_from(1.2536914064583544_f64) * t4531 + F::new(2.0) * t3273 * t8592 + F::cast_from(3.600163427964126_f64) * t8595 + F::cast_from(3.600163427964126_f64) * t8597;
    t8599
}
