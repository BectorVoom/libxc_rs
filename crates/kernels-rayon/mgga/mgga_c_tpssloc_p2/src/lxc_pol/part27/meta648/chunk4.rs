//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2242/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2242(t25471: f64, t82431: f64, t7607: f64, t82632: f64, t25490: f64, t82514: f64, t23518: f64, t7577: f64, t1014: f64, t1023: f64, t1049: f64, t12648: f64, t12652: f64, t23327: f64, t23601: f64, t23602: f64, t23605: f64, t23633: f64, t23705: f64, t23714: f64, t25429: f64, t25470: f64, t25485: f64, t25491: f64, t25492: f64, t25510: f64, t25554: f64, t25721: f64, t3041: f64, t3121: f64, t4669: f64, t4677: f64, t6743: f64, t82513: f64, t82809: f64, t89194: f64, t89205: f64) -> f64 {
    let t89445 = 0.18277045187202515961e-2_f64 * t82431 * t25471;
    let t89449 = t82632 * t7607;
    let t89468 = t82514 * t25490;
    let t89473 = t7577 * t23518;
    let t89477 = -0.36554090374405031922e-2_f64 * t82809 + t4669 * t23705 + 0.73108180748810063846e-2_f64 * t25429 * t25510 * t25721 * t12652 + 0.36554090374405031923e-2_f64 * t25429 * t25510 * t25721 * t12648 - t89445 + 0.54831135561607547884e-2_f64 * t23327 * t25470 * t23714 + 0.18277045187202515961e-2_f64 * t89449 + 0.54831135561607547884e-2_f64 * t23633 * t6743 * t4677 * t25554 - 0.16449340668482264365e-1_f64 * t23601 * t23602 * t1014 * t1049 * t25492 - 0.16449340668482264365e-1_f64 * t23601 * t25491 * t89194 * t1023 - 0.82246703342411321825e-2_f64 * t23601 * t25491 * t25485 * t3121 + 0.82246703342411321825e-2_f64 * t82513 * t89468 * t89205 * t3041 + 0.82246703342411321825e-2_f64 * t23601 * t89473 * t23605;
    t89477
}
