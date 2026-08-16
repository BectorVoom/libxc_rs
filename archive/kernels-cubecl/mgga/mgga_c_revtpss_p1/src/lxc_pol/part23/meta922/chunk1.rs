//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2980/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2980<F: Float>(t15618: F, t19785: F, t23820: F, t3153: F, t1668: F, t5825: F, t54397: F, t5819: F, t19620: F, t11774: F, t15584: F, t15689: F, t15700: F, t15701: F, t15707: F, t15758: F, t16222: F, t16226: F, t19634: F, t19639: F, t19641: F, t19702: F, t19968: F, t20075: F, t23931: F, t23934: F, t3117: F, t4808: F, t4892: F, t4894: F, t4899: F, t4900: F, t53300: F, t53318: F, t53326: F, t53800: F, t54471: F, t54570: F, t6268: F, t66565: F) -> (F, F, F, F, F) {
    let t78863 = t15618 * t19785;
    let t78873 = t23820 * t3153;
    let t78884 = t5825 * t1668;
    let t78885 = t78884 * t54397;
    let t78900 = t5819 * t1668;
    let t78901 = t78900 * t19620;
    let t78909 = F::cast_from(0.12862205435420921092e-2_f64) * t54570 * t19641 - F::cast_from(0.45732285992607719437e-2_f64) * t54471 * t6268 + F::cast_from(0.57165357490759649296e-3_f64) * t78863 - F::cast_from(0.64311027177104605458e-3_f64) * t4899 * t3117 * t66565 * t23934 + F::cast_from(0.12862205435420921092e-2_f64) * t15758 * t23931 + F::cast_from(0.76220476654346199061e-3_f64) * t53300 - t53318 + F::cast_from(0.19055119163586549765e-3_f64) * t53326 + F::cast_from(0.42874018118069736972e-3_f64) * t4892 * t3117 * t78873 * t4894 - F::cast_from(0.21437009059034868486e-3_f64) * t4899 * t3117 * t78873 * t4900 - F::cast_from(0.12862205435420921092e-2_f64) * t53800 * t20075 - F::cast_from(0.85748036236139473944e-3_f64) * t15700 * t15701 * t78885 + F::cast_from(0.85748036236139473944e-3_f64) * t16226 * t15584 * t78884 * t19634 + F::cast_from(0.7145669686344956162e-3_f64) * t15700 * t16222 * t78885 - F::cast_from(0.42874018118069736972e-3_f64) * t15689 * t15584 * t78884 * t19639 - F::cast_from(0.7145669686344956162e-3_f64) * t11774 * t16222 * t78901 + F::cast_from(0.7145669686344956162e-3_f64) * t19968 * t4808 - F::cast_from(0.42874018118069736972e-3_f64) * t15707 * t19702;
    (t78873, t78884, t78900, t78901, t78909)
}
