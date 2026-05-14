//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 906/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk906<F: Float>(t17802: F, t8850: F, t1094: F, t17449: F, t1075: F, t1067: F, t1086: F, t12223: F, t12238: F, t12265: F, t12268: F, t1472: F, t1484: F, t15434: F, t15496: F, t17750: F, t17787: F, t17790: F, t17793: F, t17803: F, t17835: F, t2974: F, t3059: F, t4087: F, t4182: F, t5123: F, t5155: F, t5158: F, t5203: F, t5219: F, t5222: F, t8765: F, t8772: F, t8786: F, t8848: F) -> (F, F, F, F) {
    let t17838 = t17802 * t8850;
    let t17843 = t17449 * t1094;
    let t17848 = t17802 * t1075;
    let t17851 = -t17750 + 0.17544670192365612213e1 * t15434 * t1484 + 0.17544670192365612213e1 * t4182 * t5219 + 0.51947267698127589899e2 * t12223 * t5222 - 0.1038945353962551798e3 * t8765 * t17787 + 0.58482233974552040708e0 * t1086 * t17790 + 0.1025389702100779493e4 * t8772 * t17793 + 3.0 * t15496 * t1472 + 3.0 * t4087 * t5155 + 0.96494049533612093922e2 * t12238 * t5158 - 0.19298809906722418785e3 * t8786 * t17803 + 1.0 * t1067 * t17835 + 0.20691336878655965246e4 * t8848 * t17838 - 0.35089340384731224426e1 * t12265 * t5203 + 0.35089340384731224426e1 * t3059 * t17843 - 6.0 * t12268 * t5123 + 6.0 * t2974 * t17848;
    (t17838, t17843, t17848, t17851)
}
