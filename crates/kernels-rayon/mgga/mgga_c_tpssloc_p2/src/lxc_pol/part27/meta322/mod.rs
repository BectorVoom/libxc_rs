//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta322 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1395;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1396;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta322(t11539: f64, t3442: f64, t1174: f64, t3247: f64, t405: f64, t974: f64, t457: f64, t63: f64, t461: f64, t221: f64, t456: f64, t1186: f64, t698: f64, t135: f64, t3471: f64, t1184: f64, t4899: f64, t3242: f64, t460: f64, t2244: f64, t3448: f64, t3469: f64, t2250: f64, t3450: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11541, t11545, t11546, t11552, t11556, t11557) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1395(t11539, t3442, t1174, t3247, t405, t974, t457, t63, t461, t221, t456, t1186, t698);
        let (t11558, t11561, t11569, t11570, t11571, t11575, t11579) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1396(t11557, t1174, t135, t3471, t1184, t4899, t3242, t460, t2244, t3448, t3469, t2250, t3450);
    (t11541, t11545, t11546, t11552, t11556, t11558, t11561, t11569, t11570, t11571, t11575, t11579)
}
