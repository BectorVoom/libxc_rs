//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2137/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2137<F: Float>(t2627: F, t7510: F, t13171: F, t1510: F, t2633: F, t6657: F, t812: F, t81599: F, t81600: F, t81718: F, t829: F, t87097: F, t87101: F, t87104: F, t87109: F, t87114: F, t87117: F, t87119: F, t87124: F, t87127: F, t87133: F, t87135: F, t87140: F) -> F {
    let t87142 = t2627 * t7510;
    let t87146 = -F::cast_from(0.82246703342411321825e-2_f64) * t87097 + t87101 + F::cast_from(0.49348022005446793095e-1_f64) * t87104 - F::cast_from(0.82246703342411321825e-2_f64) * t87109 + F::cast_from(0.9869604401089358619e-1_f64) * t87114 + F::cast_from(0.3289868133696452873e-1_f64) * t87117 - t87119 - t812 * t81718 * t1510 - F::cast_from(0.3289868133696452873e-1_f64) * t87124 - t81599 + F::cast_from(0.52089578783527170488e-1_f64) * t81600 + t87127 - t812 * t6657 * t13171 + F::cast_from(0.3289868133696452873e-1_f64) * t87133 - F::cast_from(2.0_f64) * t812 * t87135 * t829 + F::cast_from(0.16449340668482264365e-1_f64) * t87140 + F::cast_from(2.0_f64) * t812 * t87142 * t2633;
    t87146
}
