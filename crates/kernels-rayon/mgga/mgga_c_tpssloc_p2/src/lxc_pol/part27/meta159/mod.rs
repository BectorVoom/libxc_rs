//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta159 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk867;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk868;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk869;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta159(t3469: f64, t457: f64, t460: f64, t974: f64, t1184: f64, t1174: f64, t3430: f64, t3433: f64, t3436: f64, t3443: f64, t3447: f64, t3452: f64, t3457: f64, t3461: f64, t491: f64, t1190: f64, t1235: f64, t1191: f64, t225: f64, t1202: f64, t1226: f64, t3258: f64, t3261: f64, t3268: f64, t3310: f64, t3318: f64, t3408: f64, t3410: f64, t3413: f64, t3417: f64, t3421: f64, t3425: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3471, t3472, t3475, t3477, t3478, t3481) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk867(t3469, t457, t460, t974, t1184, t1174, t3430, t3433, t3436, t3443, t3447, t3452, t3457, t3461);
        let (t3482, t3484, t3487, t3490) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk868(t3481, t491, t1190, t1235, t1191, t225, t1202, t1226);
        let t3493 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk869(t3258, t3261, t3268, t3310, t3318, t3408, t3410, t3413, t3417, t3421, t3425);
    (t3471, t3472, t3475, t3477, t3478, t3481, t3482, t3484, t3487, t3490, t3493)
}
