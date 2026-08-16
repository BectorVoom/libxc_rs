//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2202/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2202(t25749: f64, t6698: f64, t7566: f64, t82573: f64, t1052: f64, t1065: f64, t11010: f64, t12648: f64, t14529: f64, t14545: f64, t23313: f64, t23329: f64, t23346: f64, t23369: f64, t25406: f64, t25429: f64, t25430: f64, t25731: f64, t25778: f64, t25811: f64, t3174: f64, t3207: f64, t4665: f64, t6687: f64, t6776: f64, t7600: f64, t82382: f64, t82432: f64, t82436: f64, t986: f64) -> f64 {
    let t88182 = t6698 * t25749;
    let t88194 = 0.14621636149762012769e-1_f64 * t82573 * t7566;
    let t88213 = 0.16449340668482264365e-1_f64 * t6687 * t986 * t88182 + 4.0_f64 * t23369 * t4665 - 0.18277045187202515961e-2_f64 * t82432 - 0.14621636149762012769e-1_f64 * t23346 * t25811 - 0.80418998823691070228e-1_f64 * t82382 * t7566 + t88194 + 0.36554090374405031923e-2_f64 * t25429 * t23329 * t25430 * t12648 - 0.82246703342411321825e-2_f64 * t6687 * t25406 * t23313 - t25778 * t3207 + 4.0_f64 * t14545 * t6776 + 2.0_f64 * t11010 * t7600 + 4.0_f64 * t1052 * t3174 * t25731 * t1065 + 4.0_f64 * t14529 * t6776 + t82436;
    t88213
}
