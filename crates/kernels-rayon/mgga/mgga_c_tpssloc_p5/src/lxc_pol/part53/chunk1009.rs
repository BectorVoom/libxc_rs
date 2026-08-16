//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1009/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1009(t1985: f64, t26202: f64, t31611: f64, t8606: f64, t8944: f64, t24994: f64, t34076: f64, t580: f64, t111: f64, t33915: f64, t116437: f64, t1983: f64, t2095: f64, t23938: f64, t24987: f64, t26880: f64, t26898: f64, t27143: f64, t32187: f64, t32189: f64, t32203: f64, t32235: f64, t33234: f64, t33363: f64, t33855: f64, t33900: f64, t34067: f64, t3701: f64, t4026: f64, t4073: f64, t55242: f64, t650: f64, t672: f64, t6876: f64, t7057: f64, t7218: f64, t7685: f64, t7687: f64, t7796: f64, t8607: f64, t8774: f64, t8805: f64, t8808: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t122562 = t1985 * t31611 * t26202;
    let t122654 = t8606 * t8944;
    let t122698 = t8606 * t24994;
    let t123337 = t34076 * t580;
    let t123368 = t33915 * t111;
    let t123373 = 2.0_f64 * t33363 * t7218 + t24987 * t8805 - t650 * t34067 + t7685 * t32187 - t4026 * t8774 + t6876 * t33855 + 6.0_f64 * t8607 * t26898 - 2.0_f64 * t8607 * t26880 + 2.0_f64 * t7685 * t32203 - 4.0_f64 * t33234 * t7057 - 4.0_f64 * t23938 * t7796 + 3.0_f64 * t1983 * t116437 * t7687 + 2.0_f64 * t1983 * t8808 * t55242 - 2.0_f64 * t1983 * t2095 * t3701 * t27143 - t7685 * t32189 - 2.0_f64 * t6876 * t33900 - 2.0_f64 * t123368 * t672 - 2.0_f64 * t32235 * t4073;
    (t122562, t122654, t122698, t123337, t123368, t123373)
}
