//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1931/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1931(t14850: f64, t3316: f64, t11282: f64, t1694: f64, t11285: f64, t3377: f64, t1164: f64, t300: f64, t4832: f64) -> (f64, f64, f64, f64, f64) {
    let t14852 = 0.16081979498692535067e2_f64 * t14850 * t3316;
    let t14853 = t11282 * t1694;
    let t14854 = t11285 * t3377;
    let t14855 = t14853 * t14854;
    let t14857 = 0.10254018858216406658e4_f64 * t1164 * t14855;
    let t14858 = t300 * t4832;
    (t14852, t14854, t14855, t14857, t14858)
}
