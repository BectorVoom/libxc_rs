//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 25/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk25<F: Float>(t12: F, t15: F, t18: F, t26: F) -> (F, F, F) {
    let t52 = F::new(0.51785e1) * t15 + F::new(0.905775e0) * t12 + F::new(0.1100325e0) * t18 + F::new(0.1241775e0) * t26;
    let t55 = F::new(1.0) + F::cast_from(0.29608574643216675549e2_f64) / t52;
    let t56 = F::ln(t55);
    (t52, t55, t56)
}
