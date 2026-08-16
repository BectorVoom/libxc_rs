//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3002/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3002(t1011: f64, t140: f64, t23873: f64, t1012: f64, t1015: f64, t11774: f64, t15584: f64, t15917: f64, t19620: f64, t19634: f64, t19639: f64, t19718: f64, t19741: f64, t19754: f64, t23837: f64, t23936: f64, t3117: f64, t43044: f64, t43050: f64, t4783: f64, t66624: f64, t66712: f64, t66714: f64, t66721: f64, t66731: f64, t66739: f64, t66747: f64, t66752: f64, t66758: f64, t66763: f64, t67528: f64, t76397: f64, t78884: f64) -> f64 {
    let t79638 = t1011 * t140 * t23873;
    let t79665 = -0.12862205435420921092e-2_f64 * t19741 * t19718 - 0.38586616306262763276e-2_f64 * t66624 * t19754 + t1011 * t1012 * t1015 * t76397 / 288.0_f64 + 7.0_f64 / 1944.0_f64 * t79638 - 0.64311027177104605458e-3_f64 * t15917 * t23936 + 0.25724410870841842184e-2_f64 * t43050 * t3117 * t23837 * t19634 - 0.12862205435420921092e-2_f64 * t43044 * t3117 * t23837 * t19639 + 0.42874018118069736972e-3_f64 * t67528 * t4783 - 0.85748036236139473944e-3_f64 * t66712 - t66714 / 108.0_f64 - t66721 / 432.0_f64 - 0.57165357490759649295e-3_f64 * t66731 + 0.28582678745379824648e-3_f64 * t66739 + 0.57165357490759649295e-3_f64 * t66747 + 0.17149607247227894789e-2_f64 * t66752 - 0.85748036236139473944e-3_f64 * t66758 - 0.19055119163586549765e-3_f64 * t66763 - 0.42874018118069736972e-3_f64 * t11774 * t15584 * t78884 * t19620;
    t79665
}
