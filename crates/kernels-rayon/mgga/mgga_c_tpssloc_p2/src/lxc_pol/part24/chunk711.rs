//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 711/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk711(t5: f64, t1860: f64, t1865: f64, t6486: f64, t6490: f64, t6492: f64, t6495: f64, t6506: f64, t6510: f64, t112: f64, t111: f64, t1868: f64) -> (f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t6514 = piecewise3(t8, 0.0_f64, -t6486 * t1865 / 6.0_f64 + 5.0_f64 / 6.0_f64 * t6490 * t6492 + t6495 * t1865 / 3.0_f64 - t1860 * t6506 / 6.0_f64 - t1860 * t6510 / 6.0_f64);
    let t6515 = t6514 * t112;
    let t6517 = t1868 * t111;
    (t6514, t6515, t6517)
}
