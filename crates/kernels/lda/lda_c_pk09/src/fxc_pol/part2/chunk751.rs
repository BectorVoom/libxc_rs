//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 751/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk751<F: Float>(t143: F, t2210: F, t2977: F, t3255: F, t3257: F, t3260: F, t3263: F, t4044: F, t7578: F, t7590: F, t7602: F, t7741: F, t7752: F, t7755: F, t7757: F, t7768: F) -> F {
    let t7771 = F::new(4.937333717448355) * t4044 * t7602 - F::new(4.937333717448355) * t2977 * t2210 + F::new(4.937333717448355) * t7741 + F::new(4.937333717448355) * t4044 * t7590 + F::new(9.87466743489671) * t4044 * t7578 + F::new(19.489173774580152) * t3255 + F::new(19.489173774580152) * t3257 - F::new(19.489173774580152) * t3260 + F::new(1.8805371096875316) * t3263 - F::new(3.159189221415045) * t7752 - F::new(3.159189221415045) * t7755 + F::new(1.4760499452555382) * t7757 + F::new(3.7610742193750633) * t143 * t7768;
    t7771
}
