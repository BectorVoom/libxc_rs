//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 880/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk880<F: Float>(t3255: F, t5428: F, t16069: F, t5425: F, t5454: F, t531: F, t5481: F, t833: F, t3761: F, t2645: F, t5452: F, t1897: F, t3754: F) -> (F, F, F, F, F, F) {
    let t16562 = F::cast_from(0.14600954814814814815e-2_f64) * t3255 * t5428;
    let t16563 = t5425 * t16069;
    let t16567 = F::cast_from(0.13140859333333333333e-2_f64) * t3255 * t5454;
    let t16568 = t5481 * t531;
    let t16569 = t16568 * t833;
    let t16570 = t3761 * t16569;
    let t16574 = t3761 * t5452 * t2645;
    let t16577 = t1897 * t3754;
    (t16562, t16563, t16567, t16570, t16574, t16577)
}
