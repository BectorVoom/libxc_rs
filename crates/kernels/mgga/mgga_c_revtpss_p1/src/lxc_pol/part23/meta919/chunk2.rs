//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2966/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2966<F: Float>(t1065: F, t24031: F, t1651: F, t4186: F, t4772: F, t6299: F, t1042: F, t1045: F, t11703: F, t11866: F, t1469: F, t15716: F, t15830: F, t15926: F, t16049: F, t16089: F, t16095: F, t18903: F, t18936: F, t18941: F, t19675: F, t19705: F, t19745: F, t19819: F, t23630: F, t23936: F, t23999: F, t2857: F, t3092: F, t3115: F, t3117: F, t3127: F, t3188: F, t4181: F, t42410: F, t4573: F, t4583: F, t4873: F, t4875: F, t4912: F, t55011: F, t55205: F, t6244: F, t6312: F, t6323: F, t65717: F, t65837: F, t67551: F, t78524: F, t906: F) -> (F, F, F) {
    let t78607 = t1065 * t24031;
    let t78616 = t1651 * t4186;
    let t78641 = t4772 * t6299;
    let t78662 = -F::cast_from(0.42874018118069736972e-3_f64) * t3127 * t1042 * t19675 * t4583 - F::cast_from(0.85748036236139473947e-3_f64) * t15716 * t1042 * t78607 * t906 - F::cast_from(0.42874018118069736972e-3_f64) * t65717 * t4875 - F::cast_from(0.64311027177104605458e-3_f64) * t55205 * t6312 - F::cast_from(0.14291339372689912324e-2_f64) * t16095 * t11703 * t4573 * t78616 - F::cast_from(0.19055119163586549765e-2_f64) * t16095 * t42410 * t18903 * t78524 + F::cast_from(0.85748036236139473944e-3_f64) * t16095 * t3092 * t18941 * t4873 - F::cast_from(0.64311027177104605458e-3_f64) * t15926 * t19745 + F::cast_from(0.34299214494455789578e-2_f64) * t16049 * t23936 + F::cast_from(0.25724410870841842183e-2_f64) * t55011 * t3092 * t18936 * t4181 - F::cast_from(0.64311027177104605458e-3_f64) * t67551 * t4912 - F::cast_from(0.64311027177104605458e-3_f64) * t11866 * t23999 - F::cast_from(0.64311027177104605458e-3_f64) * t3115 * t3117 * t78641 * t1045 - F::cast_from(0.22866142996303859718e-2_f64) * t15830 * t6323 + F::cast_from(0.85748036236139473944e-3_f64) * t3188 * t23630 - F::cast_from(0.38586616306262763276e-2_f64) * t65837 * t19819 + F::cast_from(0.17149607247227894789e-2_f64) * t16089 * t3092 * t19705 * t1469 * t4772 - F::cast_from(0.17149607247227894789e-2_f64) * t16089 * t3092 * t6244 * t2857 * t4181;
    (t78616, t78641, t78662)
}
