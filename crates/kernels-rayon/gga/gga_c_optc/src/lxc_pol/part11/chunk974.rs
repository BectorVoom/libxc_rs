//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 974/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk974(t17819: f64, t17833: f64, t1075: f64, t17802: f64, t8850: f64, t1094: f64, t17449: f64, t1067: f64, t1086: f64, t12223: f64, t12238: f64, t12265: f64, t12268: f64, t1472: f64, t1484: f64, t15434: f64, t15496: f64, t17750: f64, t17787: f64, t17790: f64, t17793: f64, t17803: f64, t2974: f64, t3059: f64, t4087: f64, t4182: f64, t5123: f64, t5155: f64, t5158: f64, t5203: f64, t5219: f64, t5222: f64, t8765: f64, t8772: f64, t8786: f64, t8848: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17834 = t17819 + t17833;
    let t17835 = t17834 * t1075;
    let t17838 = t17802 * t8850;
    let t17843 = t17449 * t1094;
    let t17848 = t17802 * t1075;
    let t17851 = -t17750 + 0.17544670192365612213e1_f64 * t15434 * t1484 + 0.17544670192365612213e1_f64 * t4182 * t5219 + 0.51947267698127589899e2_f64 * t12223 * t5222 - 0.1038945353962551798e3_f64 * t8765 * t17787 + 0.58482233974552040708e0_f64 * t1086 * t17790 + 0.1025389702100779493e4_f64 * t8772 * t17793 + 3.0_f64 * t15496 * t1472 + 3.0_f64 * t4087 * t5155 + 0.96494049533612093922e2_f64 * t12238 * t5158 - 0.19298809906722418785e3_f64 * t8786 * t17803 + 1.0_f64 * t1067 * t17835 + 0.20691336878655965246e4_f64 * t8848 * t17838 - 0.35089340384731224426e1_f64 * t12265 * t5203 + 0.35089340384731224426e1_f64 * t3059 * t17843 - 6.0_f64 * t12268 * t5123 + 6.0_f64 * t2974 * t17848;
    (t17834, t17835, t17838, t17843, t17848, t17851)
}
