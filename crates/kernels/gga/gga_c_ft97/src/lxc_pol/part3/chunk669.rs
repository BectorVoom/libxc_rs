//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 669/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk669<F: Float>(t255: F, t9802: F, t2347: F, t761: F, t731: F, t8232: F, t768: F, t9735: F, t9701: F, t251: F, t631: F, t675: F, t7242: F, t898: F) -> (F, F, F, F, F, F, F) {
    let t9803 = t9802 * t255;
    let t9808 = t761 * t2347;
    let t9822 = t8232 * t731;
    let t9824 = t8232 * t768;
    let t9863 = F::new(4.0) / F::new(27.0) * t9735;
    let t9867 = F::new(4.0) / F::new(9.0) * t9701;
    let t9890 = F::new(1.0) / t251 / t631 / t898 / t675 / t7242 / F::new(4.0);
    (t9803, t9808, t9822, t9824, t9863, t9867, t9890)
}
