//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2273/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2273(t2319: f64, t7982: f64, t12550: f64, t1266: f64, t12841: f64, t1774: f64, t24935: f64, t27290: f64, t27371: f64, t3652: f64, t4034: f64, t510: f64, t7266: f64, t7983: f64, t91564: f64, t91568: f64, t91570: f64, t91573: f64, t91578: f64, t91580: f64, t91582: f64, t91585: f64, t91587: f64, t91589: f64, t91591: f64, t91593: f64) -> (f64, f64) {
    let t94265 = t7982 * t2319;
    let t94272 = -4.0_f64 * t12550 * t7266 - 2.0_f64 * t1266 * t27371 - 2.0_f64 * t12841 * t7266 - 2.0_f64 * t1774 * t24935 - 4.0_f64 * t27290 * t4034 - t3652 * t7983 - 2.0_f64 * t510 * t94265 + t91564 + t91568 - t91570 - t91573 - t91578 - t91580 + t91582 + t91585 - t91587 - t91589 - t91591 - t91593;
    (t94265, t94272)
}
