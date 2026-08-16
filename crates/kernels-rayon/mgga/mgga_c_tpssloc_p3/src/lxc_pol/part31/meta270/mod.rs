//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta270 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1117;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1118;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1119;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta270(t1799: f64, t6968: f64, t6637: f64, t6888: f64, t5335: f64, t550: f64, t6976: f64, t1992: f64, t1834: f64, t1998: f64, t214: f64, t1985: f64, t2031: f64, t7445: f64, t5: f64, t1860: f64, t2032: f64, t7026: f64, t7034: f64, t7428: f64, t7432: f64, t7435: f64, t112: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7732, t7733, t7734, t7736, t7737, t7738, t7740, t7741, t7742) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1117(t1799, t6968, t6637, t6888, t5335, t550, t6976, t1992, t1834, t1998, t214, t1985);
        let t7782 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1118(t2031, t7445);
        let (t7786, t7787) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1119(t5, t1860, t2032, t7026, t7034, t7428, t7432, t7435, t7782, t112);
    (t7732, t7733, t7734, t7736, t7737, t7738, t7740, t7741, t7742, t7782, t7786, t7787)
}
