//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1406/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1406(t23314: f64, t23384: f64, t6707: f64, t82632: f64, t10160: f64, t10358: f64, t11007: f64, t11010: f64, t1920: f64, t1945: f64, t225: f64, t23346: f64, t23372: f64, t23399: f64, t3020: f64, t3176: f64, t345: f64, t387: f64, t388: f64, t6687: f64, t6691: f64, t6695: f64, t6768: f64, t6816: f64, t82382: f64, t83420: f64, t83424: f64, t83435: f64, t83441: f64, t83444: f64, t83453: f64, t986: f64) -> f64 {
    let t83457 = t23384 * t23314;
    let t83459 = t82632 * t6707;
    let t83461 = -6.0_f64 * t10160 * t6816 + 0.49348022005446793095e-1_f64 * t6687 * t986 * t83420 + 0.82246703342411321826e-2_f64 * t6687 * t83424 * t6691 + 6.0_f64 * t23372 * t3176 + 3.0_f64 * t3020 * t6768 * t388 + t10358 * t1945 * t388 - 0.82246703342411321826e-2_f64 * t83435 + 0.65797362673929057459e-1_f64 * t23346 * t23399 - 0.24125699647107321069e0_f64 * t82382 * t6695 - 0.14621636149762012769e-1_f64 * t83441 - 0.54831135561607547884e-2_f64 * t83444 + 0.82246703342411321825e-2_f64 * t1920 * t345 * t11007 * t225 * t387 - 3.0_f64 * t11010 * t6816 + 0.24674011002723396548e-1_f64 * t6687 * t986 * t83453 - 0.82246703342411321826e-2_f64 * t83457 + 0.54831135561607547884e-2_f64 * t83459;
    t83461
}
