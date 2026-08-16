//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2967/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2967(t15785: f64, t999: f64, t42793: f64, t4899: f64, t4901: f64, t11710: f64, t16095: f64, t16097: f64, t1011: f64, t1042: f64, t11169: f64, t11859: f64, t11883: f64, t11994: f64, t15586: f64, t15611: f64, t15725: f64, t16154: f64, t16172: f64, t1675: f64, t3117: f64, t3127: f64, t42576: f64, t42765: f64, t43066: f64, t4823: f64, t4893: f64, t4915: f64, t4920: f64, t51847: f64) -> (f64, f64) {
    let t54064 = t15785 * t999;
    let t54078 = t4899 * t42793 * t4901;
    let t54079 = 0.14291339372689912324e-3_f64 * t54078;
    let t54081 = t16095 * t11710 * t16097;
    let t54083 = -0.14291339372689912324e-3_f64 * t3127 * t1042 * t4823 * t11169 - 0.7145669686344956162e-3_f64 * t11994 * t16172 - t1011 * t4915 * t51847 / 12.0_f64 + 11.0_f64 / 81.0_f64 * t11883 * t4920 - 0.12862205435420921092e-2_f64 * t11859 * t3117 * t4893 * t54064 - 0.35400176935018568009e-1_f64 * t42576 * t1675 + 0.25724410870841842183e-2_f64 * t15725 * t16154 + 0.45732285992607719436e-2_f64 * t43066 * t15586 + 0.13719685797782315831e-1_f64 * t42765 * t15611 + t54079 + 0.11433071498151929859e-2_f64 * t54081;
    (t54064, t54083)
}
