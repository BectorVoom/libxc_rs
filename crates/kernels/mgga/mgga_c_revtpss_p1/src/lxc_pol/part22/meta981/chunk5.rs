//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3316/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3316<F: Float>(t136: F, t2457: F, t39680: F, t6022: F, t10073: F, t18746: F, t14502: F, t1559: F, t18632: F, t4366: F, t4504: F, t51332: F, t51535: F, t51537: F, t51541: F, t51544: F, t51546: F, t51550: F, t51553: F, t51560: F, t62803: F, t820: F) -> F {
    let t62907 = t39680 * t6022 * t136 * t2457;
    let t62909 = t10073 * t18746;
    let t62912 = F::cast_from(0.52683593463484092788e1_f64) * t4504 * t62803 * t4366 + F::cast_from(0.52683593463484092788e1_f64) * t4504 * t14502 * t18632 + F::cast_from(0.78059524315062264152e-1_f64) * t51535 + F::cast_from(0.29268663035268940438e-1_f64) * t51537 + F::cast_from(0.39029762157531132076e-1_f64) * t51541 + F::cast_from(0.21951497276451705328e-1_f64) * t51544 - F::cast_from(0.13170898365871023197e1_f64) * t820 * t51332 * t1559 + F::cast_from(0.52039682876708176102e-1_f64) * t51546 + F::cast_from(0.39029762157531132076e-1_f64) * t51550 - F::cast_from(0.520396828767081761e-2_f64) * t51553 + F::cast_from(0.23131639038696784277e-2_f64) * t62907 + F::cast_from(0.13009920719177044025e-2_f64) * t62909 + F::cast_from(0.92526556154787137113e-2_f64) * t51560;
    t62912
}
