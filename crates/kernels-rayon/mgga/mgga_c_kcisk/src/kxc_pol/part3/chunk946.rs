//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 946/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk946(t3883: f64, t965: f64, t13987: f64, t13989: f64, t13991: f64, t13993: f64, t13995: f64, t13998: f64, t14001: f64, t14003: f64, t14005: f64, t14008: f64, t158: f64, t173: f64) -> f64 {
    let t14011 = t965 * t3883;
    let t14013 = -0.28104e-1_f64 * t13987 - 0.32788e-1_f64 * t13989 - 0.352891875e-4_f64 * t13991 + 0.4705225e-4_f64 * t13993 + 0.50413125e-5_f64 * t173 * t13995 + 0.22405833333333333333e-5_f64 * t173 * t13998 + 0.14052e-1_f64 * t14001 - 0.4684e-2_f64 * t14003 - 0.3513e-2_f64 * t158 * t14005 + 0.78066666666666666667e-3_f64 * t158 * t14008 - 0.39624999999999999999e-2_f64 * t14011;
    t14013
}
