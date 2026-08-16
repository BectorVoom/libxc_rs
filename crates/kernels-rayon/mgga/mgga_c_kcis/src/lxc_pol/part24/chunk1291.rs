//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1291/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1291(t100378: f64, t100383: f64, t100386: f64, t100398: f64, t100401: f64, t101047: f64, t101053: f64, t18443: f64, t26748: f64, t27812: f64, t29011: f64, t4947: f64, t7703: f64, t7704: f64, t95535: f64, t95852: f64, t95855: f64) -> f64 {
    let t101189 = -0.37134344353515625e-4_f64 * t27812 * t101053 + 0.49555782539766601562e-5_f64 * t95535 * t101047 - 0.44218518518518518517e-2_f64 * t100378 + 0.99491666666666666664e-2_f64 * t100383 - 0.11054629629629629629e-2_f64 * t100386 + 0.41188271604938271605e-3_f64 * t95852 + 0.10297067901234567901e-3_f64 * t95855 - 0.23168402777777777778e-3_f64 * t7703 * t4947 * t7704 * t18443 - 0.30891203703703703704e-3_f64 * t26748 * t29011 + 0.99491666666666666664e-2_f64 * t100398 + 0.33163888888888888888e-2_f64 * t100401;
    t101189
}
