//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2128/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2128(t225: f64, t25051: f64, t23012: f64, t7489: f64, t82120: f64, t13460: f64, t1880: f64, t6553: f64, t6571: f64, t1527: f64, t23190: f64, t25160: f64, t259: f64, t2591: f64, t2718: f64, t7510: f64, t798: f64, t82108: f64, t82115: f64, t82123: f64, t855: f64, t866: f64, t86983: f64) -> f64 {
    let t86988 = t25051 * t225;
    let t86991 = t23012 * t7489;
    let t86994 = 0.3289868133696452873e-1_f64 * t82120;
    let t86997 = t1880 * t6553 * t6571 * t13460;
    let t87005 = t86983 + 2.0_f64 * t798 * t25160 * t259 - 0.24674011002723396547e-1_f64 * t82108 - 2.0_f64 * t86988 * t866 - 0.63969658155208805863e-1_f64 * t86991 - 0.76763589786250567036e-1_f64 * t82115 + t86994 - t82123 - 0.82246703342411321825e-2_f64 * t86997 + 2.0_f64 * t855 * t2718 * t23190 * t1527 + t2591 * t7510 * t259;
    t87005
}
