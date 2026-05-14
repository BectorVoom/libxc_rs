//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 899/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk899<F: Float>(t3255: F, t3271: F, t3276: F, t3250: F, t41: F, t85: F, t1106: F, t3285: F, t3265: F, t3296: F, t346: F, t9368: F, t1018: F, t127: F, t368: F, t245: F, t313: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10333 = t3255 * t3271;
    let t10335 = t3255 * t3276;
    let t10338 = t85 * t3250 * t41;
    let t10339 = t10338 * t1106;
    let t10341 = t3255 * t3285;
    let t10343 = t3255 * t3265;
    let t10351 = t3255 * t3296;
    let t10386 = t9368 * t346;
    let t10414 = t127 * t368 * t1018;
    let t10415 = t245 * t313;
    (t10333, t10335, t10338, t10339, t10341, t10343, t10351, t10386, t10414, t10415)
}
