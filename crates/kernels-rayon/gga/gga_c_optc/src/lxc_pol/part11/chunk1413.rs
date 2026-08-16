//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1413/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1413(t17451: f64, t4305: f64, t1086: f64, t1094: f64, t12223: f64, t12265: f64, t12268: f64, t1483: f64, t15381: f64, t15434: f64, t15496: f64, t17755: f64, t17758: f64, t17761: f64, t17790: f64, t17793: f64, t26217: f64, t26738: f64, t26745: f64, t3035: f64, t3059: f64, t34829: f64, t4182: f64, t44742: f64, t5155: f64, t5218: f64, t5219: f64, t5222: f64, t53108: f64, t58308: f64, t58311: f64, t58629: f64, t58784: f64, t8700: f64, t8765: f64, t8772: f64) -> (f64, f64) {
    let t59220 = 0.41015588084031179722e4_f64 * t4305 * t17451;
    let t59258 = -t58308 - 0.14035736153892489771e2_f64 * t12265 * t17755 + 0.2077890707925103596e3_f64 * t12223 * t17758 - 0.62336721237753107879e3_f64 * t8765 * t5222 * t5218 - 0.46785787179641632568e1_f64 * t3035 * t17790 * t1483 + 0.69263023597503453196e2_f64 * t3059 * t53108 * t1483 + 0.61523382126046769581e4_f64 * t8772 * t15381 * t5218 - 24.0_f64 * t12268 * t17761 + t58629 + 0.35089340384731224426e1_f64 * t15434 * t5219 + 0.1038945353962551798e3_f64 * t44742 * t5222 + 0.23392893589820816284e1_f64 * t4182 * t17790 + 0.41015588084031179722e4_f64 * t34829 * t17793 - 0.12304676425209353917e5_f64 * t26738 * t58311 * t8700 + 0.58482233974552040708e0_f64 * t1086 * t58784 * t1094 + 0.91080982599109921211e5_f64 * t26745 * t58311 * t26217 + 6.0_f64 * t15496 * t5155;
    (t59220, t59258)
}
