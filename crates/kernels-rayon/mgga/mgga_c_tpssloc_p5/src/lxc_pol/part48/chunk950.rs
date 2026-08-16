//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 950/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk950(t1888: f64, t232: f64, t6646: f64, t7084: f64, t828: f64, t22690: f64, t23171: f64, t31376: f64, t31389: f64, t6562: f64, t794: f64, t23012: f64, t8557: f64) -> (f64, f64, f64, f64) {
    let t114685 = t1888 * t6646 * t7084 * t828 * t232;
    let t114688 = t23171 * t22690 * t31376;
    let t114689 = 0.82246703342411321824e-2_f64 * t114688;
    let t114691 = t6562 * t794 * t31389;
    let t114693 = t23012 * t8557;
    (t114685, t114689, t114691, t114693)
}
