//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1018/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1018<F: Float>(t1705: F, t8692: F, t10928: F, t8697: F, t1724: F, t10925: F, t10978: F, t11119: F, t1726: F, t1735: F, t17520: F, t17562: F, t23426: F, t23430: F, t23434: F, t23438: F, t23443: F, t23447: F, t23451: F, t23492: F, t2418: F, t45: F, t4924: F, t634: F, t7091: F, t7096: F, t7135: F, t7151: F, t7182: F, t8698: F, t8733: F, t8765: F) -> (F,) {
    let t23496 = t8692 * t1705;
    let t23507 = t8697 * t10928;
    let t23508 = t23507 * t1724;
    let t23513 = -0.1025389702100779493e4 * t1735 * t23426 + 0.11696446794910408142e1 * t1735 * t23430 + 0.1038945353962551798e3 * t1735 * t23434 + 0.23392893589820816284e1 * t1735 * t23438 - 0.346315117987517266e2 * t7151 * t7182 - 0.35089340384731224426e1 * t1735 * t23443 - 0.34631511798751726598e2 * t1735 * t23447 - 0.17315755899375863299e2 * t1735 * t23451 + 0.16081824322151104822e2 * t10978 * t8733 + 0.19751789702565206229e-1 * t45 * t23492 * t634 + 1.0 * t23496 * t1726 + 2.0 * t17562 * t2418 + 2.0 * t7091 * t7135 - 2.0 * t11119 * t8698 - 0.58482233974552040708e0 * t4924 * t8765 + 0.51725014705706168417e3 * t10925 * t23508 - 4.0 * t17520 * t7096;
    (t23513,)
}
