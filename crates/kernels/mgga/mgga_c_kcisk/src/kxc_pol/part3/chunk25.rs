//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 25/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk25<F: Float>(t12: F, t15: F, t18: F, t26: F) -> (F, F, F) {
    let t52 = 0.51785e1 * t15 + 0.905775e0 * t12 + 0.1100325e0 * t18 + 0.1241775e0 * t26;
    let t55 = 1.0 + 0.29608574643216675549e2 / t52;
    let t56 = f64::ln(t55);
    (t52, t55, t56)
}
