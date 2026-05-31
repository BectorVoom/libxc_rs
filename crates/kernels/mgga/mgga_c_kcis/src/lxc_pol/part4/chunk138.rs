//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 138/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk138<F: Float>(t242: F, t245: F, t248: F, t255: F) -> (F, F, F) {
    let t401 = F::cast_from(0.705945e1_f64) * t245 + F::cast_from(0.1549425e1_f64) * t242 + F::cast_from(0.420775e0_f64) * t248 + F::cast_from(0.1562925e0_f64) * t255;
    let t404 = F::cast_from(1.0_f64) + F::cast_from(0.32164683177870697974e2_f64) / t401;
    let t405 = F::ln(t404);
    (t401, t404, t405)
}
