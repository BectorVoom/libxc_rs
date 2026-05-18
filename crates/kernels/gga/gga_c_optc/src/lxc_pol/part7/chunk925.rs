//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 925/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk925<F: Float>(t1093: F, t8791: F, t1076: F, t2968: F, t1074: F, t2976: F, t1095: F, t3053: F, t1086: F, t2930: F, t2935: F, t2969: F, t2974: F, t2977: F, t3032: F, t3035: F, t3036: F, t3054: F, t3059: F, t3062: F, t8567: F, t8754: F, t8757: F, t8762: F, t8765: F, t8766: F, t8769: F, t8772: F, t8773: F, t8776: F, t8781: F, t8786: F, t8788: F) -> (F, F, F, F, F) {
    let t8792 = t8791 * t1093;
    let t8795 = t1076 * t2968;
    let t8799 = t2968 * t2976 * t1074;
    let t8802 = t1095 * t3053;
    let t8805 = -F::new(0.35089340384731224426e1) * t8754 * t3036 + F::new(0.35089340384731224426e1) * t3059 * t8757 + F::new(0.17544670192365612213e1) * t3032 * t3054 + F::new(0.51947267698127589899e2) * t8762 * t3062 - F::new(0.1038945353962551798e3) * t8765 * t8766 + F::new(0.58482233974552040708e0) * t1086 * t8769 + F::new(0.1025389702100779493e4) * t8772 * t8773 + F::new(3.0) * t8776 * t1076 + F::new(3.0) * t2930 * t2969 + F::new(0.96494049533612093922e2) * t8781 * t2977 - F::new(0.19298809906722418785e3) * t8786 * t8788 + F::new(0.51947267698127589897e2) * t3059 * t8792 - F::new(6.0) * t2935 * t8795 + F::new(0.96494049533612093922e2) * t2974 * t8799 - F::new(0.35089340384731224426e1) * t3035 * t8802 + t8567;
    (t8792, t8795, t8799, t8802, t8805)
}
