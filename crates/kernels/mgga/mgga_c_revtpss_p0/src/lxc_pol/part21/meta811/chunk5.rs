//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2967/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2967<F: Float>(t15785: F, t999: F, t42793: F, t4899: F, t4901: F, t11710: F, t16095: F, t16097: F, t1011: F, t1042: F, t11169: F, t11859: F, t11883: F, t11994: F, t15586: F, t15611: F, t15725: F, t16154: F, t16172: F, t1675: F, t3117: F, t3127: F, t42576: F, t42765: F, t43066: F, t4823: F, t4893: F, t4915: F, t4920: F, t51847: F) -> (F, F) {
    let t54064 = t15785 * t999;
    let t54078 = t4899 * t42793 * t4901;
    let t54079 = F::cast_from(0.14291339372689912324e-3_f64) * t54078;
    let t54081 = t16095 * t11710 * t16097;
    let t54083 = -F::cast_from(0.14291339372689912324e-3_f64) * t3127 * t1042 * t4823 * t11169 - F::cast_from(0.7145669686344956162e-3_f64) * t11994 * t16172 - t1011 * t4915 * t51847 / F::new(12.0) + F::new(11.0) / F::new(81.0) * t11883 * t4920 - F::cast_from(0.12862205435420921092e-2_f64) * t11859 * t3117 * t4893 * t54064 - F::cast_from(0.35400176935018568009e-1_f64) * t42576 * t1675 + F::cast_from(0.25724410870841842183e-2_f64) * t15725 * t16154 + F::cast_from(0.45732285992607719436e-2_f64) * t43066 * t15586 + F::cast_from(0.13719685797782315831e-1_f64) * t42765 * t15611 + t54079 + F::cast_from(0.11433071498151929859e-2_f64) * t54081;
    (t54064, t54083)
}
