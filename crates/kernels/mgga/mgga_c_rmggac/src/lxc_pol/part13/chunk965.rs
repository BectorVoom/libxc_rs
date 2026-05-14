//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 965/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk965<F: Float>(t275: F, t9658: F, t41977: F, t41979: F, t36860: F, t41962: F, t41964: F, t41969: F, t41971: F, t41973: F, t41975: F, t41983: F, t41985: F, t41989: F, t41993: F, t41999: F, t42003: F, t42007: F, t5928: F, t8258: F) -> (F,) {
    let t43948 = 2.0 * t275 * t9658;
    let t43956 = 0.3193131120497015617e0 * t41977;
    let t43957 = 0.39726959900411316772e-4 * t41979;
    let t43965 = 0.1702583995731913576e-4 * t41962 - 0.1702583995731913576e-4 * t41964 + t43948 + 0.19863479950205658386e-4 * t36860 + 0.39914139006212695214e-1 * t5928 * t8258 + 0.17961362552795712846e0 * t41969 - 0.1702583995731913576e-4 * t41971 + 0.2993560425465952141e-1 * t41973 + 0.17961362552795712846e0 * t41975 + t43956 - t43957 - 0.2553875993597870364e-4 * t41983 + 0.2553875993597870364e-4 * t41985 + 0.1702583995731913576e-4 * t41989 + 0.1702583995731913576e-4 * t41993 + 0.85129199786595678799e-5 * t41999 + 0.47885174879960069324e-4 * t42003 + 0.47885174879960069324e-4 * t42007;
    (t43965,)
}
