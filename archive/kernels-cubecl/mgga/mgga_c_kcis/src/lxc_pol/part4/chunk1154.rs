//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1154/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1154<F: Float>(t14650: F, t3202: F, t14627: F, t1697: F, t2835: F, t10477: F, t14624: F, t14631: F, t14635: F, t14638: F, t14642: F, t14644: F, t14647: F, t1710: F, t2812: F, t9565: F) -> (F, F) {
    let t14651 = t3202 * t14650;
    let t14652 = t14627 * t14651;
    let t14654 = t1697 * t2835;
    let t14659 = -F::cast_from(0.2653111111111111111e-1_f64) * t14624 + F::cast_from(0.66327777777777777776e-2_f64) * t14631 - F::cast_from(0.22109259259259259258e-2_f64) * t14635 - F::cast_from(0.22109259259259259258e-2_f64) * t10477 - F::cast_from(0.33163888888888888888e-2_f64) * t14638 + F::cast_from(0.99491666666666666664e-2_f64) * t14642 + F::cast_from(0.22109259259259259258e-2_f64) * t14644 - F::cast_from(0.58958024691358024689e-2_f64) * t14647 + F::cast_from(0.11054629629629629629e-2_f64) * t14652 + F::cast_from(0.890445125e-2_f64) * t14654 * t2812 - F::cast_from(0.66725e-1_f64) * t9565 * t1710;
    (t14652, t14659)
}
