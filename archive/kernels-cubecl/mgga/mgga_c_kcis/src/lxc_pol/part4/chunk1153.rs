//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1153/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1153<F: Float>(t4802: F, t9425: F, t13132: F, t4555: F, t3210: F, t3200: F, t4797: F, t4796: F, t9438: F, t1773: F, t3217: F, t2815: F) -> (F, F, F, F, F) {
    let t14638 = t9425 * t4802;
    let t14640 = t4555 * t13132;
    let t14641 = t3210 * t14640;
    let t14642 = t3200 * t14641;
    let t14644 = t9425 * t4797;
    let t14646 = t9438 * t4796;
    let t14647 = t3200 * t14646;
    let t14649 = t3217 * t1773;
    let t14650 = t14649 * t2815;
    (t14638, t14642, t14644, t14647, t14650)
}
