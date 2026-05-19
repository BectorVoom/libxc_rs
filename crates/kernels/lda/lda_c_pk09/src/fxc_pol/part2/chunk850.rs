//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 850/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk850<F: Float>(t155: F, t205: F, t2341: F, t3758: F, t4633: F, t4643: F, t4649: F, t4652: F, t7768: F, t7776: F, t8096: F, t8101: F, t8524: F, t8731: F, t8734: F, t8744: F, t8748: F, t976: F) -> F {
    let t8757 = F::cast_from(0.29617398950766044_f64) * t8731 * t8734 - F::cast_from(7.108175748183851_f64) * t3758 * t8524 - F::cast_from(19.489173774580152_f64) * t155 * t7768 - F::cast_from(19.489173774580152_f64) * t155 * t7776 - t4633 + F::cast_from(2.3693919160612835_f64) * t205 * t8744 + F::cast_from(2.3693919160612835_f64) * t205 * t8748 - t4643 - t4649 - t4652 - F::cast_from(19.489173774580152_f64) * t155 * t8096 - F::cast_from(19.489173774580152_f64) * t155 * t8101 + F::cast_from(19.489173774580152_f64) * t976 * t2341;
    t8757
}
