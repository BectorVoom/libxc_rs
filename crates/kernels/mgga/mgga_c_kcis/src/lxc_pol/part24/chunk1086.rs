//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1086/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1086<F: Float>(t28041: F, t28043: F, t28046: F, t28048: F, t28051: F, t28053: F, t28055: F, t28057: F, t28060: F, t28062: F, t28064: F, t28066: F, t28068: F) -> F {
    let t28294 = F::new(0.26979166666666666667e-1) * t28041 + F::new(0.1875e0) * t28043 + F::new(0.625e-1) * t28046 - F::new(0.26979166666666666667e-1) * t28048 - F::new(0.9375e-1) * t28051 - F::new(0.9375e-1) * t28053 + F::new(0.10791666666666666667e0) * t28055 - F::new(0.16666666666666666667e0) * t28057 - F::new(0.9375e-1) * t28060 + F::new(0.20234375e-1) * t28062 + F::new(0.20234375e-1) * t28064 + F::new(0.25e0) * t28066 - F::new(0.625e-1) * t28068;
    t28294
}
