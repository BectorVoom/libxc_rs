//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3006/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3006<F: Float>(t23958: F, t993: F, t225: F, t366: F, t1028: F, t11703: F, t11774: F, t15584: F, t15689: F, t15696: F, t15701: F, t16095: F, t16222: F, t1665: F, t18941: F, t19725: F, t19738: F, t19773: F, t19973: F, t23857: F, t23939: F, t3092: F, t4181: F, t42155: F, t42410: F, t43082: F, t4786: F, t4854: F, t54943: F, t55011: F, t63287: F, t63363: F, t67025: F, t67044: F, t67048: F, t67072: F, t67516: F, t78524: F, t78901: F, t79097: F, t79610: F) -> (F, F, F) {
    let t79862 = t23958 * t993;
    let t79863 = t79862 * t225;
    let t79864 = t79863 * t366;
    let t79870 = F::cast_from(0.85748036236139473944e-3_f64) * t16095 * t3092 * t18941 * t78524 + F::cast_from(0.25724410870841842184e-2_f64) * t19738 * t19973 + F::cast_from(0.85748036236139473944e-3_f64) * t11774 * t15701 * t79097 - F::cast_from(0.85748036236139473944e-3_f64) * t42155 * t23939 + F::cast_from(0.25724410870841842183e-2_f64) * t67025 - F::cast_from(0.64311027177104605458e-3_f64) * t67516 * t1665 - F::cast_from(0.64311027177104605458e-3_f64) * t19773 * t4854 - F::cast_from(0.42874018118069736972e-2_f64) * t55011 * t11703 * t63363 * t4181 + F::cast_from(0.19055119163586549766e-2_f64) * t55011 * t42410 * t63287 * t4181 - F::cast_from(0.7145669686344956162e-3_f64) * t15689 * t16222 * t79610 + F::cast_from(0.85748036236139473944e-3_f64) * t11774 * t15701 * t78901 + F::cast_from(0.22866142996303859718e-2_f64) * t67044 - F::cast_from(0.85748036236139473944e-3_f64) * t67048 - F::cast_from(0.42874018118069736972e-3_f64) * t11774 * t15584 * t23857 * t4786 - F::cast_from(0.45732285992607719437e-2_f64) * t67072 - F::cast_from(0.95275595817932748825e-4_f64) * t54943 - F::cast_from(0.21437009059034868486e-3_f64) * t79864 * t1028 - F::cast_from(0.85748036236139473947e-3_f64) * t43082 * t15696 * t19725;
    (t79862, t79863, t79870)
}
