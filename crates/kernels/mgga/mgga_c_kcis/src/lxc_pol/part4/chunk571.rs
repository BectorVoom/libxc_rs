//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 571/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk571<F: Float>(t2943: F, t2944: F, t2917: F, t2919: F, t2922: F, t2925: F, t2928: F) -> (F, F, F) {
    let t2945 = t2943 * t2944;
    let t2947 = F::new(4.0) / F::new(9.0) * t2917;
    let t2952 = t2947 + F::new(2.0) / F::new(9.0) * t2919 - F::new(2.0) / F::new(9.0) * t2922 + F::new(2.0) / F::new(3.0) * t2925 - t2928 / F::new(3.0);
    (t2945, t2947, t2952)
}
