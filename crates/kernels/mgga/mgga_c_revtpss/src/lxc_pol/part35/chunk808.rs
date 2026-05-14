//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 808/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk808<F: Float>(t1868: F, t4003: F, t22046: F, t3936: F, t124: F, t22809: F, t800: F, t6816: F, t4012: F, t828: F, t1882: F, t6861: F, t9994: F, t1390: F, t1370: F, t13798: F, t13801: F, t1410: F, t22038: F, t22044: F, t22057: F, t22059: F, t22063: F, t22069: F, t4002: F, t5671: F, t9735: F, t9993: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t22841 = t4003 * t1868;
    let t22843 = t3936 * t22046 * t22841;
    let t22848 = t124 * t22809;
    let t22849 = t800 * t22848;
    let t22852 = t6816 * t1868;
    let t22854 = t4012 * t828 * t22852;
    let t22857 = t6861 * t1882;
    let t22858 = t22857 * t9994;
    let t22860 = t1390 * t828 * t22858;
    let t22863 = t22857 * t4003;
    let t22865 = t1390 * t828 * t22863;
    let t22874 = -0.51448821741683684367e-2 * t5671 * t22843 + 7.0 / 48.0 * t22038 - 7.0 / 16.0 * t22044 - t1370 * t22849 / 48.0 - t9735 + 0.12862205435420921092e-1 * t1410 * t22854 - 0.12862205435420921092e-2 * t9993 * t22860 + 0.12862205435420921092e-2 * t4002 * t22865 - 0.15246000842785598468e-3 * t22057 - 0.60023625365297631762e-1 * t22059 + 0.21437009059034868486e-4 * t22063 + 0.76230004213927992338e-3 * t22069 - 35.0 / 72.0 * t13798 + 0.30492001685571196935e-4 * t13801;
    (t22843, t22849, t22852, t22854, t22857, t22858, t22860, t22863, t22865, t22874)
}
