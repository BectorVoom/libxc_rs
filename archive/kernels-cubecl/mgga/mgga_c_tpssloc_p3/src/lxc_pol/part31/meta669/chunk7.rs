//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1984/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1984<F: Float>(t101715: F, t13397: F, t16816: F, t16830: F, t17034: F, t26657: F, t26676: F, t4182: F, t4281: F, t82032: F, t85027: F, t87687: F, t87708: F, t87718: F, t92798: F, t92810: F, t92811: F, t92822: F, t92825: F, t98601: F, t98608: F, t98881: F, t98884: F) -> F {
    let t101751 = F::cast_from(6.0_f64) * t4281 * t101715 * t4182 + t92798 - t87687 - F::cast_from(2.0_f64) * t16830 * t26676 - F::cast_from(0.52089578783527170489e-1_f64) * t82032 - F::cast_from(0.3289868133696452873e-1_f64) * t98601 - t87708 - F::cast_from(6.0_f64) * t13397 * t101715 * t16816 + t92810 + F::cast_from(4.0_f64) * t17034 * t26657 - t92811 - t85027 + F::cast_from(0.6579736267392905746e-1_f64) * t98608 - F::cast_from(0.20835831513410868196e0_f64) * t87718 + t92822 + F::cast_from(0.9869604401089358619e-1_f64) * t98881 + F::cast_from(0.82246703342411321825e-2_f64) * t98884 - t92825;
    t101751
}
