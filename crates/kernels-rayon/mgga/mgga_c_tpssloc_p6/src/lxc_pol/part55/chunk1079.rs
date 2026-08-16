//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1079/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1079(t11606: f64, t32537: f64, t24574: f64, t8872: f64, t2144: f64, t7299: f64, t7302: f64, t1186: f64, t8867: f64, t1238: f64, t2155: f64, t24589: f64, t24880: f64, t24893: f64, t32511: f64, t32516: f64, t32520: f64, t32524: f64, t32530: f64, t3487: f64, t7283: f64, t7351: f64, t7392: f64, t8888: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32538 = t11606 * t32537;
    let t32542 = 0.54831135561607547883e-2_f64 * t24574 * t8872;
    let t32543 = t7299 * t2144;
    let t32544 = t32543 * t7302;
    let t32547 = t1186 * t8867;
    let t32550 = -2.0_f64 * t24893 * t2155 + 0.3289868133696452873e-1_f64 * t7283 * t32511 + 0.16449340668482264365e-1_f64 * t7283 * t32516 - 0.54831135561607547883e-2_f64 * t7283 * t32520 + 0.54831135561607547883e-2_f64 * t24589 * t32524 - 2.0_f64 * t24880 * t2155 - 0.16449340668482264365e-1_f64 * t7283 * t32530 - 2.0_f64 * t7351 * t7392 + 2.0_f64 * t3487 * t8888 - 6.0_f64 * t1238 * t32538 - t32542 - 0.16449340668482264365e-1_f64 * t7283 * t32544 - 0.16449340668482264365e-1_f64 * t7283 * t32547;
    (t32538, t32542, t32543, t32544, t32547, t32550)
}
