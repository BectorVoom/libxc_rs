//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1027/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1027<F: Float>(t136: F, t550: F, t220: F, t124: F, t1882: F, t5675: F, t13845: F, t3924: F, t5673: F, t5674: F, t5609: F, t9794: F, t9793: F, t13817: F, t13821: F, t13826: F, t13832: F, t13834: F, t13841: F, t1410: F, t3934: F, t5671: F, t9739: F, t9742: F, t9745: F) -> (F, F, F) {
    let t13846 = t550 * t136;
    let t13847 = t13846 * t220;
    let t13848 = t124 * t1882;
    let t13850 = t13847 * t13848 * t5675;
    let t13851 = t13845 * t13850;
    let t13854 = t5673 * t5674 * t3924;
    let t13857 = t9794 * t5609;
    let t13858 = t9793 * t13857;
    let t13860 = 0.12862205435420921092e-2 * t5671 * t13817 + 0.42874018118069736972e-3 * t5671 * t13821 - 0.25724410870841842183e-1 * t1410 * t13826 - t13832 - 0.17149607247227894789e-2 * t5671 * t13834 + 0.2032800112371413129e-4 * t9739 - 35.0 / 108.0 * t9742 - 7.0 / 48.0 * t9745 + 0.85748036236139473944e-3 * t3934 * t13841 + 0.50820002809285328225e-4 * t13851 - 0.21437009059034868486e-3 * t3934 * t13854 - 0.45178982497454656791e-5 * t13858;
    (t13847, t13848, t13860)
}
