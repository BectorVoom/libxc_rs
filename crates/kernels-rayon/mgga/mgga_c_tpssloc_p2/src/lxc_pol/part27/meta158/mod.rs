//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta158 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk864;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk865;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk866;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta158(t1176: f64, t60: f64, t1184: f64, t1089: f64, t460: f64, t607: f64, t3247: f64, t461: f64, t2244: f64, t1177: f64, t1178: f64, t2250: f64, t3293: f64, t3295: f64, t3299: f64, t3302: f64, t3305: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3448 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk864(t1176, t60);
        let (t3449, t3450, t3451) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk865(t1184, t3448, t1089, t460, t607);
        let (t3452, t3456, t3457, t3460, t3461, t3464, t3469) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk866(t3449, t3451, t3247, t461, t2244, t1177, t1178, t2250, t3293, t3295, t3299, t3302, t3305);
    (t3448, t3449, t3450, t3451, t3452, t3456, t3457, t3460, t3461, t3464, t3469)
}
