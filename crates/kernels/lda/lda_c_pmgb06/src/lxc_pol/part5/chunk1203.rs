//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1203/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1203<F: Float>(t11090: F, t11093: F, t11095: F, t11098: F, t11100: F, t11101: F, t14984: F, t8647: F, t8651: F, t8655: F, t8659: F, t8668: F, t8684: F, t8685: F, t8692: F, t8693: F, t8723: F) -> F {
    let t21781 = -t8647 - t8651 + t8655 + t8659 + t8668 + F::new(3.0) * t14984 + F::new(180.0) * t11090 + t11093 + F::new(72.0) * t11095 + t11098 - t11100 - F::new(360.0) * t11101 - t8684 - F::new(1025.4018858216407) * t8685 + t8692 - F::new(0.5848223622634646) * t8693 - t8723;
    t21781
}
