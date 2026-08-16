//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta116 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk631;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk632;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk633;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk634;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk635;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk636;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk637;
use chunk7::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk638;
use chunk8::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk639;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta116(t636: f64, t2244: f64, t3240: f64, t123: f64, t2296: f64, t1088: f64, t1089: f64, t2250: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3241 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk631(t636);
        let t3242 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk632(t3241);
        let t3243 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk633(t2244, t3242);
        let (t3244, t3245) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk634(t3240, t3243, t123);
        let t3247 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk635(t2296);
        let t3248 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk636(t2244, t3247);
        let (t3249, t3250) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk637(t1088, t3248, t123);
        let t3252 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk638(t1089, t2250);
        let (t3253, t3254) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk639(t1088, t3252, t123);
    (t3241, t3242, t3243, t3244, t3245, t3247, t3248, t3249, t3250, t3252, t3253, t3254)
}
