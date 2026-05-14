//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1030/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1030<F: Float>(t14640: F, t3210: F, t3200: F, t4797: F, t9425: F, t4796: F, t9438: F, t1773: F, t3217: F, t2815: F, t3202: F, t14627: F, t1697: F, t2835: F, t10477: F, t14624: F, t14631: F, t14635: F, t14638: F, t1710: F, t2812: F, t9565: F) -> (F, F, F, F, F) {
    let t14641 = t3210 * t14640;
    let t14642 = t3200 * t14641;
    let t14644 = t9425 * t4797;
    let t14646 = t9438 * t4796;
    let t14647 = t3200 * t14646;
    let t14649 = t3217 * t1773;
    let t14650 = t14649 * t2815;
    let t14651 = t3202 * t14650;
    let t14652 = t14627 * t14651;
    let t14654 = t1697 * t2835;
    let t14659 = -0.2653111111111111111e-1 * t14624 + 0.66327777777777777776e-2 * t14631 - 0.22109259259259259258e-2 * t14635 - 0.22109259259259259258e-2 * t10477 - 0.33163888888888888888e-2 * t14638 + 0.99491666666666666664e-2 * t14642 + 0.22109259259259259258e-2 * t14644 - 0.58958024691358024689e-2 * t14647 + 0.11054629629629629629e-2 * t14652 + 0.890445125e-2 * t14654 * t2812 - 0.66725e-1 * t9565 * t1710;
    (t14642, t14644, t14647, t14652, t14659)
}
