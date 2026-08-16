//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1953/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1953(t1858: f64, t7222: f64, t1396: f64, t16546: f64, t1852: f64, t2099: f64, t24486: f64, t27286: f64, t3932: f64, t5364: f64, t5381: f64, t7223: f64, t7240: f64, t7961: f64, t84031: f64, t85394: f64, t85397: f64, t91830: f64, t91832: f64, t91834: f64) -> f64 {
    let t91842 = 2.0_f64 * t7222 * t1858;
    let t91846 = 2.0_f64 * t1396 * t27286 + t16546 * t2099 + t1852 * t24486 + t3932 * t7961 + 2.0_f64 * t5364 * t7240 + 2.0_f64 * t5381 * t7223 + t84031 + t85394 + 2.0_f64 * t85397 + t91830 + t91832 + t91834 + t91842;
    t91846
}
