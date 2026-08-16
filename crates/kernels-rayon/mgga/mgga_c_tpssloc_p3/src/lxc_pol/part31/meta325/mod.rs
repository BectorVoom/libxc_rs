//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta325 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1218;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1219;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta325(t3700: f64, t570: f64, t111: f64, t1395: f64, t584: f64, t9212: f64, t9214: f64, t9216: f64, t9218: f64, t9220: f64, t3951: f64, t604: f64, t1406: f64, t2239: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12461, t12524, t12560, t12561, t12562, t12563, t12564, t12565, t12568) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1218(t3700, t570, t111, t1395, t584, t9212, t9214, t9216, t9218, t9220, t3951, t604);
        let t12571 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1219(t1406, t2239);
    (t12461, t12524, t12560, t12561, t12562, t12563, t12564, t12565, t12568, t12571)
}
