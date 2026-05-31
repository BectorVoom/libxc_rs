//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 916/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk916<F: Float>(t662: F, t8734: F, t646: F, t2337: F, t644: F, t14: F, t2340: F, t31: F, t8663: F, t666: F, t671: F, t8674: F, t8678: F, t8682: F, t8700: F, t8704: F, t8708: F, t8713: F, t8717: F, t8725: F) -> (F, F, F) {
    let t8735 = t8734 * t662;
    let t8737 = F::cast_from(1.0_f64) * t646 * t8735;
    let t8739 = F::cast_from(1.0_f64) / t2337 / t644;
    let t8740 = t14 * t8739;
    let t8742 = F::cast_from(1.0_f64) / t2340 / t31;
    let t8743 = t8663 * t8742;
    let t8745 = F::cast_from(0.51725014705706168417e3_f64) * t8740 * t8743;
    let t8746 = t8674 + t8678 + F::cast_from(0.1038945353962551798e3_f64) * t671 * t8682 - F::cast_from(0.58482233974552040708e0_f64) * t671 * t8700 - F::cast_from(0.35089340384731224426e1_f64) * t671 * t8704 + F::cast_from(0.35089340384731224426e1_f64) * t671 * t8708 - F::cast_from(0.51947267698127589897e2_f64) * t671 * t8713 - F::cast_from(0.56969282336565386482e-3_f64) * t666 * t8717 - t8725 + t8737 + t8745;
    (t8737, t8745, t8746)
}
