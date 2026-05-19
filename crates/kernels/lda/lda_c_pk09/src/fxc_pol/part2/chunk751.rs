//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 751/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk751<F: Float>(t143: F, t2210: F, t2977: F, t3255: F, t3257: F, t3260: F, t3263: F, t4044: F, t7578: F, t7590: F, t7602: F, t7741: F, t7752: F, t7755: F, t7757: F, t7768: F) -> F {
    let t7771 = F::cast_from(4.937333717448355_f64) * t4044 * t7602 - F::cast_from(4.937333717448355_f64) * t2977 * t2210 + F::cast_from(4.937333717448355_f64) * t7741 + F::cast_from(4.937333717448355_f64) * t4044 * t7590 + F::cast_from(9.87466743489671_f64) * t4044 * t7578 + F::cast_from(19.489173774580152_f64) * t3255 + F::cast_from(19.489173774580152_f64) * t3257 - F::cast_from(19.489173774580152_f64) * t3260 + F::cast_from(1.8805371096875316_f64) * t3263 - F::cast_from(3.159189221415045_f64) * t7752 - F::cast_from(3.159189221415045_f64) * t7755 + F::cast_from(1.4760499452555382_f64) * t7757 + F::cast_from(3.7610742193750633_f64) * t143 * t7768;
    t7771
}
