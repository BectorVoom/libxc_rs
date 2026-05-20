//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2982/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2982<F: Float>(t19697: F, t4820: F, t1011: F, t1042: F, t1063: F, t11656: F, t11859: F, t11875: F, t11927: F, t15707: F, t16012: F, t16067: F, t16208: F, t1671: F, t19620: F, t19639: F, t19649: F, t19792: F, t23892: F, t23992: F, t23997: F, t3092: F, t3095: F, t3117: F, t3155: F, t4583: F, t4837: F, t4866: F, t53692: F, t53944: F, t6263: F, t6271: F, t65342: F, t65567: F, t65570: F, t65581: F, t65585: F, t77564: F, t77568: F, t77573: F, t78496: F, t78790: F) -> F {
    let t78986 = t19697 * t4820;
    let t79006 = F::cast_from(0.14291339372689912324e-3_f64) * t16067 * t3092 * t78496 * t3095 - F::cast_from(0.25724410870841842184e-2_f64) * t11859 * t3117 * t6271 * t3155 * t4866 + F::cast_from(0.47637797908966374413e-3_f64) * t65567 - F::cast_from(0.57165357490759649295e-3_f64) * t65570 - F::new(7.0) / F::new(54.0) * t1011 * t16012 * t77564 + F::new(7.0) / F::new(216.0) * t1011 * t16012 * t77568 + F::new(35.0) / F::new(972.0) * t1011 * t53944 * t77573 + F::cast_from(0.19055119163586549766e-2_f64) * t1063 * t1042 * t16208 * t78790 - F::cast_from(0.34299214494455789578e-2_f64) * t65342 * t1671 + F::cast_from(0.22866142996303859718e-2_f64) * t11656 * t23892 + F::cast_from(0.42874018118069736972e-3_f64) * t78986 + F::cast_from(0.85748036236139473944e-3_f64) * t4837 * t1042 * t19649 * t4583 - F::cast_from(0.85748036236139473944e-3_f64) * t53692 * t6263 - F::cast_from(0.85748036236139473944e-3_f64) * t15707 * t19792 + F::cast_from(0.64311027177104605458e-3_f64) * t11875 * t3117 * t23992 * t19639 + F::cast_from(0.12862205435420921092e-2_f64) * t11927 * t3117 * t23997 * t19620 + F::cast_from(0.14291339372689912324e-3_f64) * t65581 + F::cast_from(0.85748036236139473944e-3_f64) * t65585;
    t79006
}
