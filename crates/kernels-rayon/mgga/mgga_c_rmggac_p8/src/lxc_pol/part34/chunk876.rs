//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 876/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk876(t14374: f64, t15322: f64, t69924: f64, t570: f64, t68740: f64, t1550: f64, t14207: f64, t2868: f64, t2001: f64, t305: f64, t3141: f64, t8580: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t75664 = t14374 * t15322;
    let t75666 = 0.19863479950205658386e-4_f64 * t69924;
    let t75674 = t68740 * t570;
    let t75675 = t1550 * t75674;
    let t75677 = t2868 * t14207;
    let t75678 = 0.79828278012425390427e-1_f64 * t75677;
    let t75681 = t3141 * t2001 * t305 * t8580;
    (t75664, t75666, t75674, t75675, t75678, t75681)
}
