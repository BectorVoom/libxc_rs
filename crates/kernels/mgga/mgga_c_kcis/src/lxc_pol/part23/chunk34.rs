//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 34/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk34<F: Float>(t6: F, t73: F, t69: F, t63: F, t66: F, t21: F, t2: F) -> (F, F, F, F, F, F) {
    let t74 = t6 * t73;
    let t75 = t69 * t74;
    let t78 = F::new(1.0) + t63 * t66 * t75 / F::new(96.0);
    let t79 = F::ln(t78);
    let t81 = F::new(1.0) + F::new(0.66725e-1) * t79;
    let t82 = F::new(1.0) / t81;
    let t84 = F::new(1.0) / t21;
    let t85 = t2 * t84;
    (t74, t75, t78, t81, t82, t85)
}
