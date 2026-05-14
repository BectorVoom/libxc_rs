//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1208/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1208<F: Float>(t1085: F, t1094: F, t1102: F, t26556: F, t2968: F, t2936: F, t1075: F, t26152: F, t26156: F, t26160: F, t26163: F, t26164: F, t26192: F, t26200: F, t26203: F, t26206: F, t26209: F, t26212: F, t26229: F, t2974: F, t2976: F, t3035: F, t3059: F, t3061: F, t8757: F, t8762: F, t8765: F, t8781: F, t8786: F, t8809: F) -> (F, F, F, F) {
    let t26560 = 0.58482233974552040708e0 * t1102 * t1085 * t26556 * t1094;
    let t26561 = t2968 * t2968;
    let t26578 = t2936 * t2936;
    let t26582 = 0.96494049533612093922e2 * t2974 * t26561 * t2976 + 0.14035736153892489771e2 * t8762 * t8757 - 0.1403573615389248977e2 * t8765 * t26164 * t1094 - 0.35089340384731224426e1 * t3035 * t26229 * t1094 + 0.51947267698127589897e2 * t3059 * t26229 * t3061 + 24.0 * t8781 * t8809 - 24.0 * t8786 * t26578 * t1075 - t26152 + t26156 + t26160 - t26163 - t26192 - t26200 + t26203 + t26206 - t26209 - t26212;
    (t26560, t26561, t26578, t26582)
}
