//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 891/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk891<F: Float>(t1527: F, t4459: F, t507: F, t4462: F, t515: F, t14758: F, t1524: F, t4435: F, t1537: F, t1197: F, t3696: F, t1212: F, t12885: F, t14728: F, t14733: F, t14736: F, t14737: F, t14740: F, t14743: F, t14744: F, t14747: F, t14752: F, t14757: F, t14759: F, t14793: F, t1529: F, t1538: F, t1542: F, t1543: F, t4431: F, t4438: F, t4456: F, t4461: F, t4464: F, t4468: F, t4472: F, t4475: F, t4478: F, t4479: F) -> (F,) {
    let t14797 = 1.0 / t4459 / t1527;
    let t14798 = t507 * t14797;
    let t14800 = 1.0 / t4462 / t515;
    let t14801 = t14758 * t14800;
    let t14804 = t1524 * t4435;
    let t14807 = t14758 * t1537;
    let t14810 = t1197 * t3696;
    let t14813 = t12885 * t1212;
    let t14816 = 0.17544670192365612213e1 * t14728 * t1543 + 0.17544670192365612213e1 * t4468 * t4475 + 0.51947267698127589899e2 * t14733 * t4479 - 0.1038945353962551798e3 * t14736 * t14737 + 0.58482233974552040708e0 * t1542 * t14740 + 0.1025389702100779493e4 * t14743 * t14744 + 3.0 * t14747 * t1538 + 3.0 * t4431 * t4456 + 0.96494049533612093922e2 * t14752 * t4464 - 0.19298809906722418785e3 * t14757 * t14759 + 1.0 * t1529 * t14793 + 0.20691336878655965246e4 * t14798 * t14801 - 6.0 * t14804 * t4438 + 6.0 * t4461 * t14807 - 0.35089340384731224426e1 * t14810 * t4472 + 0.35089340384731224426e1 * t4478 * t14813;
    (t14816,)
}
