//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1373/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1373(t23384: f64, t23715: f64, t210: f64, t23632: f64, t23668: f64, t225: f64, t82390: f64, t23518: f64, t6733: f64, t23628: f64, t6680: f64, t10305: f64, t10316: f64, t10321: f64, t23323: f64, t23346: f64, t23601: f64, t23605: f64, t23637: f64, t23657: f64, t23673: f64, t23685: f64, t23687: f64, t25713: f64, t2780: f64, t3016: f64, t6687: f64, t6784: f64, t6785: f64, t6787: f64, t6797: f64, t6805: f64, t6806: f64, t82382: f64, t82385: f64) -> f64 {
    let t82661 = t23384 * t23715;
    let t82668 = t23668 * t210 * t23632;
    let t82676 = t82390 * t225;
    let t82683 = t6733 * t23518;
    let t82694 = t6680 * t23628;
    let t82705 = -0.54831135561607547883e-2_f64 * t82661 + 0.27415567780803773942e-2_f64 * t6687 * t6784 * t6785 * t10321 - 0.43864908449286038307e-1_f64 * t82668 * t23637 + 0.24125699647107321069e0_f64 * t23323 * t6806 - 0.24674011002723396548e-1_f64 * t6797 * t23657 * t23673 + 0.8529287754027840782e-2_f64 * t6687 * t82676 * t6785 * t10305 + 0.80418998823691070229e-1_f64 * t82382 * t6787 + 0.24674011002723396548e-1_f64 * t23601 * t82683 * t23605 - 0.24674011002723396548e-1_f64 * t6687 * t82385 * t25713 + 0.82246703342411321826e-2_f64 * t6687 * t6784 * t23685 * t2780 - 0.43864908449286038307e-1_f64 * t82694 + 0.16449340668482264365e-1_f64 * t6687 * t6784 * t6785 * t10316 - 0.24674011002723396548e-1_f64 * t6687 * t3016 * t6805 - 0.43864908449286038307e-1_f64 * t23346 * t23687;
    t82705
}
