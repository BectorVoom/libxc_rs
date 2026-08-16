//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 934/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk934(t10012: f64, t2684: f64, t2925: f64, t9438: f64, t3005: f64, t9800: f64, t9829: f64, t13142: f64, t7416: f64, t10054: f64, t3040: f64, t3267: f64, t8556: f64) -> (f64, f64, f64, f64, f64) {
    let t44001 = t2684 * t9438 * t10012 * t2925;
    let t44002 = 0.15976219147466979032e-1_f64 * t44001;
    let t44004 = t9800 * t3005 * t9829;
    let t44005 = 0.19171462976960374838e1_f64 * t44004;
    let t44009 = t7416 * t13142;
    let t44010 = 0.15976219147466979032e-1_f64 * t44009;
    let t44027 = 0.35750489951850426669e0_f64 * t10054 * t3040;
    let t44029 = 0.23833659967900284446e0_f64 * t3267 * t8556;
    (t44002, t44005, t44010, t44027, t44029)
}
