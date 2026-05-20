//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3003/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3003<F: Float>(t1043: F, t1668: F, t357: F, t11660: F, t11703: F, t11774: F, t11933: F, t15618: F, t15758: F, t15917: F, t16067: F, t16095: F, t18936: F, t19501: F, t19572: F, t19645: F, t19776: F, t19778: F, t19782: F, t19971: F, t19992: F, t19997: F, t23470: F, t23474: F, t23900: F, t23904: F, t23994: F, t24007: F, t3091: F, t3092: F, t3117: F, t4186: F, t42410: F, t43050: F, t43082: F, t4786: F, t4873: F, t4892: F, t4899: F, t53511: F, t54578: F, t54599: F, t6268: F, t65144: F, t66689: F, t66814: F) -> (F, F) {
    let t79703 = t1668 * t1043 * t357;
    let t79723 = F::cast_from(0.95275595817932748825e-4_f64) * t54599 + F::cast_from(0.42874018118069736972e-3_f64) * t15618 * t19645 + F::cast_from(0.7145669686344956162e-3_f64) * t15618 * t19782 + F::cast_from(0.14291339372689912324e-3_f64) * t3091 * t3092 * t23474 * t4786 + F::cast_from(0.63517063878621832552e-3_f64) * t3091 * t42410 * t23470 * t4786 + F::cast_from(0.25724410870841842184e-2_f64) * t43050 * t3117 * t24007 * t53511 + F::cast_from(0.34299214494455789578e-2_f64) * t11933 * t23994 - F::cast_from(0.71456696863449561621e-3_f64) * t16095 * t11703 * t18936 * t4873 + F::cast_from(0.12862205435420921092e-2_f64) * t4892 * t3117 * t19572 * t19971 - F::cast_from(0.85748036236139473944e-3_f64) * t66814 + F::cast_from(0.17149607247227894789e-2_f64) * t11774 * t66689 * t19992 - F::cast_from(0.17149607247227894789e-2_f64) * t43082 * t66689 * t19997 + F::cast_from(0.85748036236139473944e-3_f64) * t54578 * t6268 + F::cast_from(0.64311027177104605458e-3_f64) * t16067 * t3117 * t65144 * t79703 + F::cast_from(0.85748036236139473944e-3_f64) * t15758 * t23900 + F::cast_from(0.85748036236139473944e-3_f64) * t4892 * t3092 * t19501 * t11660 * t4186 - F::cast_from(0.42874018118069736972e-3_f64) * t15917 * t23904 - F::cast_from(0.42874018118069736972e-3_f64) * t4899 * t3092 * t19501 * t19776 + F::cast_from(0.85748036236139473944e-3_f64) * t15618 * t19778;
    (t79703, t79723)
}
