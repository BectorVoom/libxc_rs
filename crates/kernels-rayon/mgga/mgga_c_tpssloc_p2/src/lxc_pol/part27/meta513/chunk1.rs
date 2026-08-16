//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1918/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1918(t225: f64, t7577: f64, t6786: f64, t1539: f64, t23685: f64, t6784: f64, t23657: f64, t7610: f64, t23327: f64, t23346: f64, t23619: f64, t23626: f64, t23629: f64, t25456: f64, t25459: f64, t25465: f64, t25467: f64, t6687: f64, t6797: f64, t7607: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25470 = t7577 * t225;
    let t25471 = t25470 * t6786;
    let t25475 = t23685 * t1539;
    let t25476 = t6784 * t25475;
    let t25479 = t23657 * t7610;
    let t25482 = -0.82246703342411321825e-2_f64 * t6687 * t25456 - 0.82246703342411321825e-2_f64 * t6687 * t25459 - t23619 - 0.73108180748810063845e-2_f64 * t23626 + 0.21932454224643019153e-1_f64 * t23346 * t7607 - 0.27415567780803773942e-2_f64 * t25465 - 0.82246703342411321825e-2_f64 * t6687 * t25467 - 0.27415567780803773942e-2_f64 * t23327 * t25471 + 0.27415567780803773942e-2_f64 * t23629 + 0.27415567780803773942e-2_f64 * t6687 * t25476 - 0.82246703342411321825e-2_f64 * t6797 * t25479;
    (t25470, t25471, t25475, t25476, t25479, t25482)
}
