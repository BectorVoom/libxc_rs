//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1290/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1290<F: Float>(t11418: F, t1616: F, t27607: F, t28778: F, t54162: F, t7978: F, t8225: F, t27594: F, t6140: F, t16694: F, t18183: F, t27583: F, t27598: F, t28714: F, t28835: F, t8226: F, t94905: F, t94966: F, t95021: F, t98193: F, t98201: F, t99004: F) -> F {
    let t99120 = t1616 * t11418;
    let t99129 = F::new(0.23168402777777777778e-3) * t27607 * t28778;
    let t99131 = t7978 * t54162 * t8225;
    let t99133 = t27594 * t6140;
    let t99144 = -F::new(0.36039737654320987655e-3) * t27583 * t18183 * t99120 * t16694 + F::new(0.185671721767578125e-4) * t94966 * t99004 - F::new(0.41270617283950617282e-2) * t98193 + t99129 - F::new(0.7722800925925925926e-4) * t99131 + F::new(0.24756229569010416667e-4) * t99133 * t27598 - F::new(0.30945286961263020833e-5) * t94905 + F::new(0.69644166666666666664e-2) * t98201 - F::new(0.69505208333333333334e-3) * t28714 * t27598 + F::new(0.34752604166666666667e-3) * t95021 * t8226 + F::new(0.69505208333333333334e-3) * t27607 * t28835;
    t99144
}
