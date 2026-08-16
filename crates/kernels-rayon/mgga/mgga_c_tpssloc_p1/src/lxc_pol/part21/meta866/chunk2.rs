//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3162/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3162(t11877: f64, t11881: f64, t11904: f64, t11907: f64, t1215: f64, t1244: f64, t1246: f64, t19120: f64, t19128: f64, t19131: f64, t19145: f64, t19146: f64, t19154: f64, t19165: f64, t19176: f64, t19189: f64, t19201: f64, t3493: f64, t3507: f64, t3610: f64, t3621: f64, t44753: f64, t44754: f64, t45329: f64, t5068: f64, t5069: f64, t52485: f64, t6238: f64, t6252: f64, t6257: f64) -> f64 {
    let t65408 = 2.0_f64 * t1215 * t1244 * t1246 * t19120 + t1244 * t1246 * t3493 * t6238 + 14.0_f64 * t3507 * t44753 * t44754 * t6252 + 12.0_f64 * t11881 * t19145 * t19165 + 4.0_f64 * t19128 * t3610 * t5068 + 4.0_f64 * t19189 * t3610 * t5068 + 2.0_f64 * t11877 * t6257 + 4.0_f64 * t11904 * t19176 - 4.0_f64 * t11907 * t19131 - 2.0_f64 * t11907 * t19146 + 2.0_f64 * t19154 * t45329 + t19201 * t3621 + 8.0_f64 * t5069 * t52485;
    t65408
}
