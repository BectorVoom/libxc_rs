//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta273 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1033;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1034;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta273(t1372: f64, t3752: f64, t1376: f64, t68: f64, t1385: f64, t3888: f64, t3911: f64, t3887: f64, t225: f64, t3753: f64, t3880: f64, t1323: f64, t3879: f64, t522: f64, t9212: f64, t9214: f64, t3824: f64, t592: f64, t11976: f64, t11978: f64, t11980: f64, t11982: f64, t11984: f64, t9457: f64, t9476: f64, t9484: f64, t9780: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12016, t12019, t12021, t12023, t12027, t12030, t12033, t12036) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1033(t1372, t3752, t1376, t68, t1385, t3888, t3911, t3887, t225, t3753, t3880, t1323, t3879);
        let (t12044, t12046, t12048, t12049) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1034(t522, t9212, t9214, t3824, t592, t11976, t11978, t11980, t11982, t11984, t9457, t9476, t9484, t9780);
    (t12016, t12019, t12021, t12023, t12027, t12030, t12033, t12036, t12044, t12046, t12048, t12049)
}
