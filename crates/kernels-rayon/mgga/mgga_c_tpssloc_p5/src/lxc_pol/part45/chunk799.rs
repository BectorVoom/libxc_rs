//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 799/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk799(t23664: f64, t23720: f64, t1055: f64, t1065: f64, t6815: f64, t3174: f64, t2780: f64, t6690: f64, t6689: f64, t10170: f64, t1052: f64, t11010: f64, t1956: f64, t23579: f64, t23582: f64, t23589: f64, t23595: f64, t3026: f64, t6680: f64, t6687: f64, t6700: f64, t6816: f64) -> f64 {
    let t23721 = t23664 + t23720;
    let t23722 = t1055 * t23721;
    let t23724 = t6815 * t1065;
    let t23725 = t3174 * t23724;
    let t23728 = t6690 * t2780;
    let t23729 = t6689 * t23728;
    let t23732 = -t10170 * t1956 - 0.43864908449286038306e-1_f64 * t6680 * t6700 - t11010 * t1956 + 0.18277045187202515961e-2_f64 * t23579 + 0.54831135561607547884e-2_f64 * t6687 * t23582 - 2.0_f64 * t3026 * t6816 + 0.16449340668482264365e-1_f64 * t6687 * t23589 + 0.36554090374405031923e-2_f64 * t6687 * t23595 - t1052 * t23722 + 4.0_f64 * t1052 * t23725 + 0.27415567780803773942e-2_f64 * t6687 * t23729;
    t23732
}
