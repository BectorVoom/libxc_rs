//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 773/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk773(t221: f64, t3829: f64, t9921: f64, t3978: f64, t3970: f64, t3989: f64, t1388: f64, t3934: f64, t4002: f64, t5671: f64, t9828: f64, t9832: f64, t9837: f64, t9842: f64, t9847: f64, t9893: f64, t9896: f64, t9901: f64, t9906: f64, t9910: f64, t9914: f64, t9919: f64) -> (f64, f64) {
    let t9923 = t9921 * t221 * t3829;
    let t9924 = t3978 * t9923;
    let t9926 = t3989 * t3970;
    let t9928 = 0.25724410870841842183e-2_f64 * t3934 * t9828 - 0.64311027177104605458e-3_f64 * t3934 * t9832 - 0.51448821741683684367e-2_f64 * t5671 * t9837 + 0.12862205435420921092e-2_f64 * t5671 * t9842 + 0.76230004213927992336e-5_f64 * t9847 - 0.21437009059034868486e-3_f64 * t1388 * t9893 + 0.30011812682648815881e-2_f64 * t9896 - 0.21437009059034868486e-3_f64 * t1388 * t9901 - 0.38115002106963996168e-4_f64 * t9906 - 0.17006693853500995666e-1_f64 * t9910 + 0.12862205435420921092e-2_f64 * t4002 * t9914 - 0.60023625365297631762e-2_f64 * t9919 + 0.76230004213927992338e-3_f64 * t9924 + 0.12004725073059526352e-1_f64 * t9926;
    (t9923, t9928)
}
