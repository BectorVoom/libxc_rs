//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1240/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1240<F: Float>(t19945: F, t19948: F, t19951: F, t19954: F, t19958: F, t19961: F, t19963: F, t19965: F, t19967: F, t19970: F, t20131: F, t20134: F, t20137: F, t20139: F, t20143: F, t20146: F, t20149: F, t20152: F) -> F {
    let t20789 = F::new(0.26979166666666666667e-1) * t19945 + F::new(0.29976851851851851851e-2) * t19948 + F::new(0.125e0) * t19951 + F::new(0.1875e0) * t19954 + F::new(0.625e-1) * t19958 - F::new(0.4046875e-1) * t19961 + F::new(0.20234375e-1) * t19963 - F::new(0.20833333333333333333e-1) * t19965 - F::new(0.26979166666666666666e-1) * t19967 - F::new(0.16666666666666666667e0) * t19970 + F::new(0.9375e-1) * t20131 - F::new(0.5e0) * t20134 + F::new(0.375e0) * t20137 - F::new(0.33333333333333333333e0) * t20139 - F::new(0.9375e-1) * t20143 + F::new(0.101171875e-1) * t20146 + F::new(0.25e0) * t20149 - F::new(0.41666666666666666667e-1) * t20152;
    t20789
}
