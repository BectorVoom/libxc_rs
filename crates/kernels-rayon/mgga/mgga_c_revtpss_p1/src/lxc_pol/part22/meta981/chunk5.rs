//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3316/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3316(t136: f64, t2457: f64, t39680: f64, t6022: f64, t10073: f64, t18746: f64, t14502: f64, t1559: f64, t18632: f64, t4366: f64, t4504: f64, t51332: f64, t51535: f64, t51537: f64, t51541: f64, t51544: f64, t51546: f64, t51550: f64, t51553: f64, t51560: f64, t62803: f64, t820: f64) -> f64 {
    let t62907 = t39680 * t6022 * t136 * t2457;
    let t62909 = t10073 * t18746;
    let t62912 = 0.52683593463484092788e1_f64 * t4504 * t62803 * t4366 + 0.52683593463484092788e1_f64 * t4504 * t14502 * t18632 + 0.78059524315062264152e-1_f64 * t51535 + 0.29268663035268940438e-1_f64 * t51537 + 0.39029762157531132076e-1_f64 * t51541 + 0.21951497276451705328e-1_f64 * t51544 - 0.13170898365871023197e1_f64 * t820 * t51332 * t1559 + 0.52039682876708176102e-1_f64 * t51546 + 0.39029762157531132076e-1_f64 * t51550 - 0.520396828767081761e-2_f64 * t51553 + 0.23131639038696784277e-2_f64 * t62907 + 0.13009920719177044025e-2_f64 * t62909 + 0.92526556154787137113e-2_f64 * t51560;
    t62912
}
