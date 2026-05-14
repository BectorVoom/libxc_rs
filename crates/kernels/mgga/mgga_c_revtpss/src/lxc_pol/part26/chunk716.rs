//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 716/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk716<F: Float>(t4003: F, t9898: F, t1390: F, t828: F, t4000: F, t820: F, t843: F, t4006: F, t136: F, t4011: F, t221: F, t3829: F, t3978: F, t3970: F, t3989: F, t1388: F, t3934: F, t4002: F, t5671: F, t9828: F, t9832: F, t9837: F, t9842: F, t9847: F, t9893: F, t9896: F, t9901: F, t9906: F, t9910: F) -> (F, F, F, F) {
    let t9912 = t9898 * t4003;
    let t9914 = t1390 * t828 * t9912;
    let t9918 = t820 * t4000 * t843;
    let t9919 = t9918 * t4006;
    let t9921 = t4011 * t136;
    let t9923 = t9921 * t221 * t3829;
    let t9924 = t3978 * t9923;
    let t9926 = t3989 * t3970;
    let t9928 = 0.25724410870841842183e-2 * t3934 * t9828 - 0.64311027177104605458e-3 * t3934 * t9832 - 0.51448821741683684367e-2 * t5671 * t9837 + 0.12862205435420921092e-2 * t5671 * t9842 + 0.76230004213927992336e-5 * t9847 - 0.21437009059034868486e-3 * t1388 * t9893 + 0.30011812682648815881e-2 * t9896 - 0.21437009059034868486e-3 * t1388 * t9901 - 0.38115002106963996168e-4 * t9906 - 0.17006693853500995666e-1 * t9910 + 0.12862205435420921092e-2 * t4002 * t9914 - 0.60023625365297631762e-2 * t9919 + 0.76230004213927992338e-3 * t9924 + 0.12004725073059526352e-1 * t9926;
    (t9912, t9914, t9923, t9928)
}
