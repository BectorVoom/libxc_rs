//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1022/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1022(t111: f64, t32348: f64, t114387: f64, t114388: f64, t114405: f64, t114413: f64, t114415: f64, t115821: f64, t2039: f64, t2363: f64, t23917: f64, t24932: f64, t27888: f64, t32350: f64, t671: f64, t7056: f64, t7266: f64, t85428: f64, t94248: f64, t96222: f64) -> (f64, f64) {
    let t117533 = t32348 * t111;
    let t117550 = 4.0_f64 * t117533 * t671 + 2.0_f64 * t2039 * t85428 + 2.0_f64 * t2039 * t94248 + 4.0_f64 * t2039 * t96222 + 2.0_f64 * t2363 * t32350 + 2.0_f64 * t23917 * t7266 + 4.0_f64 * t24932 * t7056 + 4.0_f64 * t27888 * t7056 + t114387 + t114388 + t114405 + t114413 + t114415 + t115821;
    (t117533, t117550)
}
