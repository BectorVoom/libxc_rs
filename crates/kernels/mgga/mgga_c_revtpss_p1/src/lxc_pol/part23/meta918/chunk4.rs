//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2963/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2963<F: Float>(t11703: F, t11859: F, t16052: F, t16081: F, t16089: F, t16095: F, t18908: F, t19634: F, t19758: F, t19831: F, t20096: F, t20101: F, t23481: F, t23900: F, t23992: F, t2852: F, t2857: F, t3091: F, t3092: F, t3117: F, t4181: F, t43254: F, t4757: F, t4786: F, t4891: F, t4896: F, t4902: F, t54500: F, t54570: F, t6100: F, t6258: F, t65288: F, t65292: F, t65298: F, t66766: F, t67725: F, t67790: F, t78496: F, t78512: F, t78524: F) -> F {
    let t78545 = -F::cast_from(0.14291339372689912324e-2_f64) * t3091 * t11703 * t23481 * t4786 + F::cast_from(0.85748036236139473947e-3_f64) * t16081 * t3092 * t78496 * t43254 - F::cast_from(0.45732285992607719437e-2_f64) * t16052 * t23900 - F::cast_from(0.12862205435420921092e-2_f64) * t11859 * t3117 * t23992 * t19634 - F::cast_from(0.2540682555144873302e-2_f64) * t65288 - F::cast_from(0.15879265969655458138e-3_f64) * t65292 - F::cast_from(0.25724410870841842183e-2_f64) * t65298 + F::cast_from(0.12862205435420921092e-2_f64) * t54500 * t19831 - F::cast_from(0.85748036236139473947e-3_f64) * t78512 + F::cast_from(0.12862205435420921092e-2_f64) * t67725 * t4891 * t4896 - F::cast_from(0.64311027177104605458e-3_f64) * t67790 * t4891 * t4902 + F::cast_from(0.85748036236139473944e-3_f64) * t16089 * t3092 * t6100 * t4757 + F::cast_from(0.42874018118069736972e-2_f64) * t16095 * t11703 * t18908 * t78524 + F::cast_from(0.17149607247227894789e-2_f64) * t66766 * t20096 - F::cast_from(0.14291339372689912324e-2_f64) * t66766 * t20101 + F::cast_from(0.64311027177104605458e-3_f64) * t54570 * t19758 + F::cast_from(0.85748036236139473944e-3_f64) * t16095 * t3092 * t6258 * t2857 * t4181 - F::cast_from(0.7145669686344956162e-3_f64) * t16095 * t11703 * t6258 * t2852 * t4181;
    t78545
}
