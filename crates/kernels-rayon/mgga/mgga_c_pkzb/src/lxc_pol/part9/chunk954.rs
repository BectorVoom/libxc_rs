//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 954/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk954(t7431: f64, t7442: f64, t684: f64, t664: f64, t2793: f64, t694: f64, t7335: f64, t5522: f64, t7332: f64, t7352: f64, t7361: f64, t7363: f64, t7366: f64, t7368: f64, t7371: f64, t7373: f64, t7376: f64, t7379: f64) -> (f64, f64, f64, f64, f64) {
    let t7443 = t7431 + t7442;
    let t7444 = t7443 * t684;
    let t7446 = 1.0_f64 * t664 * t7444;
    let t7447 = t2793 * t694;
    let t7451 = 0.60385e0_f64 * t7335;
    let t7462 = 0.27595e0_f64 * t7332 - t7451 + 0.905775e0_f64 * t7352 + 0.16504875e0_f64 * t7361 + 0.258925e1_f64 * t7363 - 0.258925e1_f64 * t7366 - 0.1294625e1_f64 * t7368 + 0.16504875e0_f64 * t7371 + 0.82524375e-1_f64 * t7373 + 0.19419375e1_f64 * t7376 - 0.412621875e-1_f64 * t7379 + 0.80513333333333333334e0_f64 * t5522;
    (t7443, t7444, t7446, t7447, t7462)
}
