//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1122/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1122(t13847: f64, t13848: f64, t5675: f64, t13845: f64, t3924: f64, t5673: f64, t5674: f64, t5609: f64, t9794: f64, t9793: f64, t13817: f64, t13821: f64, t13826: f64, t13832: f64, t13834: f64, t13841: f64, t1410: f64, t3934: f64, t5671: f64, t9739: f64, t9742: f64, t9745: f64) -> f64 {
    let t13850 = t13847 * t13848 * t5675;
    let t13851 = t13845 * t13850;
    let t13854 = t5673 * t5674 * t3924;
    let t13857 = t9794 * t5609;
    let t13858 = t9793 * t13857;
    let t13860 = 0.12862205435420921092e-2_f64 * t5671 * t13817 + 0.42874018118069736972e-3_f64 * t5671 * t13821 - 0.25724410870841842183e-1_f64 * t1410 * t13826 - t13832 - 0.17149607247227894789e-2_f64 * t5671 * t13834 + 0.2032800112371413129e-4_f64 * t9739 - 35.0_f64 / 108.0_f64 * t9742 - 7.0_f64 / 48.0_f64 * t9745 + 0.85748036236139473944e-3_f64 * t3934 * t13841 + 0.50820002809285328225e-4_f64 * t13851 - 0.21437009059034868486e-3_f64 * t3934 * t13854 - 0.45178982497454656791e-5_f64 * t13858;
    t13860
}
