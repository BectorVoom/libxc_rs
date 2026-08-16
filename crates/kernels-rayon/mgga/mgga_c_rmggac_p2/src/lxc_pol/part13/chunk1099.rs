//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1099/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1099(t275: f64, t9658: f64, t41977: f64, t41979: f64, t36860: f64, t41962: f64, t41964: f64, t41969: f64, t41971: f64, t41973: f64, t41975: f64, t41983: f64, t41985: f64, t41989: f64, t41993: f64, t41999: f64, t42003: f64, t42007: f64, t5928: f64, t8258: f64) -> f64 {
    let t43948 = 2.0_f64 * t275 * t9658;
    let t43956 = 0.3193131120497015617e0_f64 * t41977;
    let t43957 = 0.39726959900411316772e-4_f64 * t41979;
    let t43965 = 0.1702583995731913576e-4_f64 * t41962 - 0.1702583995731913576e-4_f64 * t41964 + t43948 + 0.19863479950205658386e-4_f64 * t36860 + 0.39914139006212695214e-1_f64 * t5928 * t8258 + 0.17961362552795712846e0_f64 * t41969 - 0.1702583995731913576e-4_f64 * t41971 + 0.2993560425465952141e-1_f64 * t41973 + 0.17961362552795712846e0_f64 * t41975 + t43956 - t43957 - 0.2553875993597870364e-4_f64 * t41983 + 0.2553875993597870364e-4_f64 * t41985 + 0.1702583995731913576e-4_f64 * t41989 + 0.1702583995731913576e-4_f64 * t41993 + 0.85129199786595678799e-5_f64 * t41999 + 0.47885174879960069324e-4_f64 * t42003 + 0.47885174879960069324e-4_f64 * t42007;
    t43965
}
