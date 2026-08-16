//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 827/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk827(t1089: f64, t2090: f64, t4643: f64, t598: f64, t1967: f64, t2299: f64, t2294: f64, t7840: f64, t7845: f64, t7847: f64, t7850: f64, t7854: f64, t7863: f64, t7864: f64, t8963: f64, t8967: f64, t8971: f64, t8973: f64, t8975: f64) -> (f64, f64) {
    let t8978 = t1089 * t4643 * t2090;
    let t8979 = t598 * t8978;
    let t8981 = t1967 * t2299;
    let t8983 = t1967 * t2294;
    let t8989 = 0.31448092289604152068e-3_f64 * t8963 - 0.47172138434406228102e-3_f64 * t8967 + 0.15724046144802076034e-3_f64 * t8971 + 0.32155513588552302729e-2_f64 * t8973 - 0.28303283060643736861e-2_f64 * t8975 - 0.21437009059034868486e-3_f64 * t8979 - 0.47172138434406228102e-2_f64 * t8981 + 0.12862205435420921092e-2_f64 * t8983 + 0.15724046144802076034e-3_f64 * t7840 + 0.10482697429868050689e-3_f64 * t7845 - 0.10718504529517434243e-3_f64 * t7847 + t7850 + t7854 + t7863 - 7.0_f64 / 288.0_f64 * t7864;
    (t8978, t8989)
}
