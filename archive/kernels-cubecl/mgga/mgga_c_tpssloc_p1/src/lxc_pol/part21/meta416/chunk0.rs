//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1932/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1932<F: Float>(t1166: F, t14858: F, t3419: F, t4869: F, t11180: F, t1671: F, t3259: F, t4782: F, t14704: F, t14710: F, t14722: F, t11215: F, t11217: F, t14720: F, t14733: F, t14738: F, t14742: F, t14746: F, t14751: F, t14755: F, t14766: F) -> (F, F, F, F, F, F, F, F) {
    let t14860 = F::cast_from(0.11696447245269292414e1_f64) * t14858 * t1166;
    let t14862 = F::cast_from(0.5848223622634646207e0_f64) * t4869 * t3419;
    let t14864 = F::cast_from(1.0_f64) * t11180 * t1671;
    let t14866 = F::cast_from(2.0_f64) * t3259 * t4782;
    let t14868 = F::cast_from(0.19931111111111111111e0_f64) * t14704;
    let t14870 = F::cast_from(0.10954222222222222222e0_f64) * t14710;
    let t14886 = F::cast_from(0.39862222222222222222e0_f64) * t14722;
    let t14887 = -F::cast_from(0.10954222222222222222e0_f64) * t11215 - F::cast_from(0.54771111111111111111e-1_f64) * t11217 + F::cast_from(0.91285185185185185185e-1_f64) * t14766 + F::cast_from(0.13287407407407407408e0_f64) * t14720 - F::cast_from(0.39862222222222222222e0_f64) * t14738 - F::cast_from(0.19931111111111111111e0_f64) * t14742 - F::cast_from(0.11958666666666666667e1_f64) * t14733 + F::cast_from(0.11958666666666666667e1_f64) * t14751 + F::cast_from(0.59793333333333333334e0_f64) * t14755 + F::cast_from(0.17938e1_f64) * t14746 - t14886;
    (t14860, t14862, t14864, t14866, t14868, t14870, t14886, t14887)
}
