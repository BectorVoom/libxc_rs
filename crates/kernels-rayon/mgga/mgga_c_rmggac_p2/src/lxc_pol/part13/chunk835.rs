//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 835/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk835(t194: f64, t1979: f64, t1982: f64, t201: f64, t5530: f64, t2134: f64, t27: f64, t3118: f64, t551: f64, t2350: f64, t4905: f64, t26283: f64) -> (f64, f64, f64, f64) {
    let t38780 = t194 * t5530 * t201 * t1979 * t1982;
    let t38784 = t2134 * t27 * t3118 * t551;
    let t38792 = t2350 * t4905;
    let t38793 = t26283 * t38792;
    (t38780, t38784, t38792, t38793)
}
