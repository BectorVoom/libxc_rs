//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2982/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2982(t19697: f64, t4820: f64, t1011: f64, t1042: f64, t1063: f64, t11656: f64, t11859: f64, t11875: f64, t11927: f64, t15707: f64, t16012: f64, t16067: f64, t16208: f64, t1671: f64, t19620: f64, t19639: f64, t19649: f64, t19792: f64, t23892: f64, t23992: f64, t23997: f64, t3092: f64, t3095: f64, t3117: f64, t3155: f64, t4583: f64, t4837: f64, t4866: f64, t53692: f64, t53944: f64, t6263: f64, t6271: f64, t65342: f64, t65567: f64, t65570: f64, t65581: f64, t65585: f64, t77564: f64, t77568: f64, t77573: f64, t78496: f64, t78790: f64) -> f64 {
    let t78986 = t19697 * t4820;
    let t79006 = 0.14291339372689912324e-3_f64 * t16067 * t3092 * t78496 * t3095 - 0.25724410870841842184e-2_f64 * t11859 * t3117 * t6271 * t3155 * t4866 + 0.47637797908966374413e-3_f64 * t65567 - 0.57165357490759649295e-3_f64 * t65570 - 7.0_f64 / 54.0_f64 * t1011 * t16012 * t77564 + 7.0_f64 / 216.0_f64 * t1011 * t16012 * t77568 + 35.0_f64 / 972.0_f64 * t1011 * t53944 * t77573 + 0.19055119163586549766e-2_f64 * t1063 * t1042 * t16208 * t78790 - 0.34299214494455789578e-2_f64 * t65342 * t1671 + 0.22866142996303859718e-2_f64 * t11656 * t23892 + 0.42874018118069736972e-3_f64 * t78986 + 0.85748036236139473944e-3_f64 * t4837 * t1042 * t19649 * t4583 - 0.85748036236139473944e-3_f64 * t53692 * t6263 - 0.85748036236139473944e-3_f64 * t15707 * t19792 + 0.64311027177104605458e-3_f64 * t11875 * t3117 * t23992 * t19639 + 0.12862205435420921092e-2_f64 * t11927 * t3117 * t23997 * t19620 + 0.14291339372689912324e-3_f64 * t65581 + 0.85748036236139473944e-3_f64 * t65585;
    t79006
}
