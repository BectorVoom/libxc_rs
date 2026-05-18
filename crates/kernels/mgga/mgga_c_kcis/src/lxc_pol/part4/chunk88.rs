//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 88/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk88<F: Float>(t250: F, t252: F, t253: F, t242: F, t245: F, t248: F) -> (F, F, F, F) {
    let t255 = t250 * t252 * t253;
    let t257 = F::new(0.379785e1) * t245 + F::new(0.8969e0) * t242 + F::new(0.204775e0) * t248 + F::new(0.123235e0) * t255;
    let t260 = F::new(1.0) + F::new(0.16081824322151104822e2) / t257;
    let t261 = f64::ln(t260);
    (t255, t257, t260, t261)
}
