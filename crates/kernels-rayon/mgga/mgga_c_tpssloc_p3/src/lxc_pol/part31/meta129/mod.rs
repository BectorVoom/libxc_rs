//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta129 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk697;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk698;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk699;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta129(t3242: f64, t461: f64, t337: f64, t51: f64, t1887: f64, t1176: f64, t60: f64, t1184: f64, t1089: f64, t460: f64, t607: f64, t3247: f64, t3293: f64, t1191: f64, t225: f64, t1202: f64, t1226: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3441, t3447) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk697(t3242, t461, t337, t51, t1887);
        let t3448 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk698(t1176, t60);
        let (t3449, t3450, t3451, t3455, t3464, t3487, t3490) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk699(t1184, t3448, t1089, t460, t607, t3247, t461, t3293, t1191, t225, t1202, t1226);
    (t3441, t3447, t3448, t3449, t3450, t3451, t3455, t3464, t3487, t3490)
}
