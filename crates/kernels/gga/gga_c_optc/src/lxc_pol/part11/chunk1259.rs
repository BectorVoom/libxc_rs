//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1259/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1259<F: Float>(t1102: F, t14871: F, t15374: F, t17529: F, t4305: F, t3061: F, t58311: F, t8697: F, t17451: F, t1086: F, t1094: F, t12223: F, t12265: F, t12268: F, t1483: F, t15381: F, t15434: F, t15496: F, t17755: F, t17758: F, t17761: F, t17790: F, t17793: F, t26217: F, t26738: F, t26745: F, t3035: F, t3059: F, t34829: F, t4182: F, t44742: F, t5155: F, t5218: F, t5219: F, t5222: F, t53108: F, t58308: F, t58629: F, t58784: F, t8700: F, t8765: F, t8772: F) -> (F, F, F, F, F) {
    let t59212 = 0.62336721237753107879e3 * t1102 * t14871 * t15374;
    let t59214 = 0.14035736153892489771e2 * t4305 * t17529;
    let t59218 = 0.6233672123775310788e3 * t1102 * t8697 * t58311 * t3061;
    let t59220 = 0.41015588084031179722e4 * t4305 * t17451;
    let t59258 = -t58308 - 0.14035736153892489771e2 * t12265 * t17755 + 0.2077890707925103596e3 * t12223 * t17758 - 0.62336721237753107879e3 * t8765 * t5222 * t5218 - 0.46785787179641632568e1 * t3035 * t17790 * t1483 + 0.69263023597503453196e2 * t3059 * t53108 * t1483 + 0.61523382126046769581e4 * t8772 * t15381 * t5218 - 24.0 * t12268 * t17761 + t58629 + 0.35089340384731224426e1 * t15434 * t5219 + 0.1038945353962551798e3 * t44742 * t5222 + 0.23392893589820816284e1 * t4182 * t17790 + 0.41015588084031179722e4 * t34829 * t17793 - 0.12304676425209353917e5 * t26738 * t58311 * t8700 + 0.58482233974552040708e0 * t1086 * t58784 * t1094 + 0.91080982599109921211e5 * t26745 * t58311 * t26217 + 6.0 * t15496 * t5155;
    (t59212, t59214, t59218, t59220, t59258)
}
