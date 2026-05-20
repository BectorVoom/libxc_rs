//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3002/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3002<F: Float>(t1011: F, t140: F, t23873: F, t1012: F, t1015: F, t11774: F, t15584: F, t15917: F, t19620: F, t19634: F, t19639: F, t19718: F, t19741: F, t19754: F, t23837: F, t23936: F, t3117: F, t43044: F, t43050: F, t4783: F, t66624: F, t66712: F, t66714: F, t66721: F, t66731: F, t66739: F, t66747: F, t66752: F, t66758: F, t66763: F, t67528: F, t76397: F, t78884: F) -> F {
    let t79638 = t1011 * t140 * t23873;
    let t79665 = -F::cast_from(0.12862205435420921092e-2_f64) * t19741 * t19718 - F::cast_from(0.38586616306262763276e-2_f64) * t66624 * t19754 + t1011 * t1012 * t1015 * t76397 / F::new(288.0) + F::new(7.0) / F::new(1944.0) * t79638 - F::cast_from(0.64311027177104605458e-3_f64) * t15917 * t23936 + F::cast_from(0.25724410870841842184e-2_f64) * t43050 * t3117 * t23837 * t19634 - F::cast_from(0.12862205435420921092e-2_f64) * t43044 * t3117 * t23837 * t19639 + F::cast_from(0.42874018118069736972e-3_f64) * t67528 * t4783 - F::cast_from(0.85748036236139473944e-3_f64) * t66712 - t66714 / F::new(108.0) - t66721 / F::new(432.0) - F::cast_from(0.57165357490759649295e-3_f64) * t66731 + F::cast_from(0.28582678745379824648e-3_f64) * t66739 + F::cast_from(0.57165357490759649295e-3_f64) * t66747 + F::cast_from(0.17149607247227894789e-2_f64) * t66752 - F::cast_from(0.85748036236139473944e-3_f64) * t66758 - F::cast_from(0.19055119163586549765e-3_f64) * t66763 - F::cast_from(0.42874018118069736972e-3_f64) * t11774 * t15584 * t78884 * t19620;
    t79665
}
