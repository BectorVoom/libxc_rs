//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1443/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1443<F: Float>(t13847: F, t13848: F, t5675: F, t13845: F, t3924: F, t5673: F, t5674: F, t5609: F, t9794: F, t9793: F, t13817: F, t13821: F, t13826: F, t13832: F, t13834: F, t13841: F, t1410: F, t3934: F, t5671: F, t9739: F, t9742: F, t9745: F) -> (F, F, F, F) {
    let t13850 = t13847 * t13848 * t5675;
    let t13851 = t13845 * t13850;
    let t13854 = t5673 * t5674 * t3924;
    let t13857 = t9794 * t5609;
    let t13858 = t9793 * t13857;
    let t13860 = F::cast_from(0.12862205435420921092e-2_f64) * t5671 * t13817 + F::cast_from(0.42874018118069736972e-3_f64) * t5671 * t13821 - F::cast_from(0.25724410870841842183e-1_f64) * t1410 * t13826 - t13832 - F::cast_from(0.17149607247227894789e-2_f64) * t5671 * t13834 + F::cast_from(0.2032800112371413129e-4_f64) * t9739 - F::cast_from(35.0_f64) / F::cast_from(108.0_f64) * t9742 - F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t9745 + F::cast_from(0.85748036236139473944e-3_f64) * t3934 * t13841 + F::cast_from(0.50820002809285328225e-4_f64) * t13851 - F::cast_from(0.21437009059034868486e-3_f64) * t3934 * t13854 - F::cast_from(0.45178982497454656791e-5_f64) * t13858;
    (t13850, t13854, t13857, t13860)
}
