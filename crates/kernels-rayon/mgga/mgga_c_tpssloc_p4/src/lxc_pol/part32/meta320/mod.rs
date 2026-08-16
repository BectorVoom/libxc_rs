//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta320 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1348;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1349;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta320(t3247: f64, t405: f64, t974: f64, t457: f64, t63: f64, t461: f64, t221: f64, t456: f64, t1186: f64, t698: f64, t1174: f64, t1184: f64, t4899: f64, t3242: f64, t460: f64, t1176: f64, t134: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11545, t11546, t11552, t11553, t11556, t11558, t11569) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1348(t3247, t405, t974, t457, t63, t461, t221, t456, t1186, t698, t1174, t1184, t4899);
        let (t11570, t11583, t11588) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1349(t3242, t460, t3247, t1176, t134);
    (t11545, t11546, t11552, t11553, t11556, t11558, t11569, t11570, t11583, t11588)
}
