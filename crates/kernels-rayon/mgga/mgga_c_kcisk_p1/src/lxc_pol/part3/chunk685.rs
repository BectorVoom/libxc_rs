//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 685/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk685(t10634: f64, t10680: f64, t1676: f64, t1685: f64, t10542: f64, t10543: f64, t10549: f64, t10554: f64, t10559: f64, t10563: f64, t10566: f64, t10602: f64, t10604: f64, t1674: f64, t1686: f64, t4757: f64, t4783: f64) -> (f64, f64, f64) {
    let t10681 = t10634 + t10680;
    let t10683 = t1676 * t10681 * t1685;
    let t10686 = t10542 - 0.17544670192365612213e1_f64 * t10543 * t1686 - 0.17544670192365612213e1_f64 * t4757 * t4783 - 0.51947267698127589897e2_f64 * t1674 * t10549 - 0.35089340384731224426e1_f64 * t1674 * t10554 - t10559 + t10563 - t10566 - t10602 + 0.35089340384731224426e1_f64 * t1674 * t10604 - 0.58482233974552040708e0_f64 * t1674 * t10683;
    (t10681, t10683, t10686)
}
