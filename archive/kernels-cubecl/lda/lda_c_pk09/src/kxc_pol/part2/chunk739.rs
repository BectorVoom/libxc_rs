//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 739/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk739<F: Float>(t741: F, t93: F, t89: F, t623: F, t7647: F, t121: F, t2214: F, t2222: F, t2977: F, t3138: F, t3142: F, t3265: F, t709: F, t713: F, t7590: F, t7619: F, t7621: F, t7625: F, t7629: F, t7634: F, t7636: F, t7640: F, t7642: F, t7650: F) -> F {
    let t7652 = t93 * t741;
    let t7653 = t89 * t7652;
    let t7654 = t7647 * t623;
    let t7655 = t121 * t7654;
    let t7658 = F::cast_from(1.8805371096875316_f64) * t3265 * t7590 + F::cast_from(2.9824072957409817_f64) * t2222 * t3138 + F::cast_from(2.9824072957409817_f64) * t2222 * t3142 - F::cast_from(0.15277772349540736_f64) * t7619 * t7621 + F::cast_from(5.9648145914819635_f64) * t7625 * t7621 + F::cast_from(2.9824072957409817_f64) * t7629 - F::cast_from(4.937333717448355_f64) * t2977 * t2214 - F::cast_from(4.937333717448355_f64) * t7634 * t7636 - F::cast_from(4.937333717448355_f64) * t7640 - F::cast_from(4.937333717448355_f64) * t7642 * t713 - F::cast_from(4.937333717448355_f64) * t7642 * t709 - F::cast_from(0.04115066352984959_f64) * t7650 - F::cast_from(0.08230132705969918_f64) * t7653 * t7655;
    t7658
}
