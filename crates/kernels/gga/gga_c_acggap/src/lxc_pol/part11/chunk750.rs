//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 750/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk750<F: Float>(t7864: F, t1090: F, t1181: F, t604: F, t7575: F, t1096: F, t1165: F, t7351: F, t7806: F, t7809: F, t7813: F, t7817: F, t7820: F, t7823: F, t7825: F, t7829: F, t7833: F, t7837: F, t7840: F, t7845: F, t7848: F, t7850: F, t7854: F, t7856: F, t7863: F) -> (F, F, F) {
    let t7865 = F::new(7.0) / F::new(144.0) * t7864;
    let t7867 = t1181 * t604 * t1090;
    let t7868 = t7575 * t7867;
    let t7871 = t1165 * t7351 * t1096;
    let t7872 = t7575 * t7871;
    let t7874 = -t7806 + F::cast_from(0.114609375e-1_f64) * t7809 + F::new(0.7640625e-2) * t7813 + t7817 / F::new(64.0) + F::new(0.22921875e-1) * t7820 - F::cast_from(0.17149607247227894789e-2_f64) * t7823 + F::cast_from(0.17149607247227894789e-2_f64) * t7825 - t7829 / F::new(128.0) + F::cast_from(0.15724046144802076034e-3_f64) * t7833 + F::cast_from(0.21437009059034868486e-3_f64) * t7837 + F::cast_from(0.31448092289604152068e-3_f64) * t7840 + F::cast_from(0.20965394859736101378e-3_f64) * t7845 - t7848 + t7850 + t7854 + t7856 / F::new(96.0) + t7863 - t7865 + F::cast_from(0.31448092289604152068e-2_f64) * t7868 - F::cast_from(0.47172138434406228102e-2_f64) * t7872;
    (t7867, t7871, t7874)
}
