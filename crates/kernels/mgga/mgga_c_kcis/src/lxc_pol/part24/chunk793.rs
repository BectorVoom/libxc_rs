//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 793/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk793<F: Float>(t4682: F, t930: F, t1666: F, t2985: F, t4711: F, t659: F, t4708: F, t13714: F, t1676: F, t2331: F, t22: F, t4864: F) -> (F, F, F, F, F, F, F, F) {
    let t13867 = t4682 * t930;
    let t13872 = t1666 * t2985;
    let t13908 = t659 * t4711;
    let t13909 = F::new(0.21908444444444444444e0) * t13908;
    let t13912 = t659 * t4708;
    let t13939 = F::new(0.39862222222222222222e0) * t13714;
    let t13945 = t2331 * t1676;
    let t13948 = t22 * t4864;
    (t13867, t13872, t13908, t13909, t13912, t13939, t13945, t13948)
}
