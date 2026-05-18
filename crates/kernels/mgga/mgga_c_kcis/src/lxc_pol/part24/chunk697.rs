//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 697/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk697<F: Float>(t8067: F, t8070: F, t8073: F, t8075: F, t8077: F, t8079: F) -> F {
    let t8117 = F::new(0.9375e-1) * t8067 - F::new(0.9375e-1) * t8070 + F::new(0.625e-1) * t8073 - F::new(0.20234375e-1) * t8075 + F::new(0.20234375e-1) * t8077 - F::new(0.26979166666666666667e-1) * t8079;
    t8117
}
