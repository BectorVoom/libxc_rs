//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1288/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1288(t100970: f64, t13097: f64, t26686: f64, t100204: f64, t100208: f64, t100212: f64, t100219: f64, t100229: f64, t100983: f64, t101084: f64, t26685: f64, t26695: f64, t27812: f64, t27826: f64, t27832: f64, t71203: f64, t7703: f64, t95764: f64, t9933: f64) -> (f64, f64) {
    let t101101 = t26686 * t13097 * t100970;
    let t101104 = -0.33163888888888888888e-2_f64 * t100204 - 0.185671721767578125e-4_f64 * t27812 * t101084 + 0.55273148148148148147e-3_f64 * t100208 + 0.73697530864197530862e-3_f64 * t100212 - 0.33163888888888888888e-2_f64 * t100219 - 0.36848765432098765431e-3_f64 * t100229 + 0.92673611111111111112e-3_f64 * t27832 * t27826 + 0.30891203703703703704e-3_f64 * t7703 * t9933 * t26695 * t71203 - 0.20594135802469135803e-3_f64 * t95764 + 0.37101880208333333334e-3_f64 * t26685 * t100983 + 0.55652820312500000001e-3_f64 * t26685 * t101101;
    (t101101, t101104)
}
