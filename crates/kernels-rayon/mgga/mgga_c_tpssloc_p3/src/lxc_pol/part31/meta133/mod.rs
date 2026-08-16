//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta133 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk707;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk708;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk709;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta133(t1216: f64, t248: f64, t3570: f64, t1213: f64, t478: f64, t483: f64, t3068: f64, t1244: f64, t1230: f64, t820: f64, t1089: f64, t415: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t3572, t3573, t3575, t3576, t3577) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk707(t1216, t248, t3570, t1213, t478, t483, t3068, t1244);
        let t3578 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk708(t1230, t820);
        let t3584 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk709(t1089, t415);
    (t3572, t3573, t3575, t3576, t3577, t3578, t3584)
}
