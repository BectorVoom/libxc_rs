//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta276 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1280;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1281;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta276(t3941: f64, t7769: f64, t1401: f64, t7467: f64, t1409: f64, t1419: f64, t56: f64, t6503: f64, t7251: f64, t67: f64, t1864: f64, t2109: f64, t7445: f64, t5: f64, t1860: f64, t2110: f64, t7246: f64, t7428: f64, t7432: f64, t7435: f64, t112: f64, t1458: f64, t2165: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7771, t7773, t7973, t7974, t7975, t7978) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1280(t3941, t7769, t1401, t7467, t1409, t1419, t56, t6503, t7251, t67, t1864, t2109, t7445);
        let (t7982, t7983, t7989) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1281(t5, t1860, t2110, t7246, t7428, t7432, t7435, t7975, t7978, t112, t1458, t2165);
    (t7771, t7773, t7973, t7974, t7975, t7978, t7982, t7983, t7989)
}
