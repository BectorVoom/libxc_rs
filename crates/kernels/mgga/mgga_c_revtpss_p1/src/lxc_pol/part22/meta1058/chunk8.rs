//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3761/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3761<F: Float>(t21107: F, t3704: F, t17628: F, t5373: F, t16750: F, t1794: F, t1042: F, t1250: F, t12976: F, t16746: F, t17237: F, t17351: F, t17426: F, t17569: F, t17589: F, t20952: F, t21085: F, t21111: F, t3647: F, t3667: F, t3711: F, t3718: F, t3720: F, t5047: F, t5277: F, t5333: F, t5391: F, t59233: F, t59239: F, t6647: F, t71245: F) -> (F, F) {
    let t71710 = t21107 * t3704;
    let t71718 = t5373 * t17628;
    let t71724 = t16750 * t1794;
    let t71737 = F::cast_from(0.57165357490759649296e-3_f64) * t17569 * t17589 - F::cast_from(0.1270341277572436651e-2_f64) * t3647 * t21111 - F::cast_from(0.30488190661738479624e-2_f64) * t71710 + F::cast_from(0.28582678745379824648e-3_f64) * t3711 * t1042 * t5277 * t16746 + F::cast_from(0.67751534803863288053e-2_f64) * t5391 * t17237 - t71718 / F::new(243.0) + F::cast_from(0.1270341277572436651e-3_f64) * t59233 + F::cast_from(0.15244095330869239812e-2_f64) * t59239 + F::cast_from(0.17149607247227894789e-2_f64) * t17426 * t20952 - F::cast_from(0.42874018118069736972e-3_f64) * t3718 * t3720 * t71724 * t1250 - F::cast_from(0.95275595817932748826e-3_f64) * t17351 * t71245 * t5333 * t5047 - F::cast_from(0.21437009059034868486e-3_f64) * t12976 * t6647 - F::cast_from(0.42874018118069736972e-3_f64) * t3667 * t21085;
    (t71724, t71737)
}
