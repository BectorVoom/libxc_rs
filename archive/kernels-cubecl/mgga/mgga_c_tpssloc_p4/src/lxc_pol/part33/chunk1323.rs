//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1323/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1323<F: Float>(t105445: F, t105449: F, t105453: F, t105462: F, t1528: F, t17090: F, t1912: F, t28307: F, t4147: F, t67344: F, t7538: F, t82123: F, t82154: F, t98166: F, t98322: F) -> F {
    let t105466 = -t82123 - F::cast_from(0.49348022005446793095e-1_f64) * t105445 + F::cast_from(0.14804406601634037928e0_f64) * t105449 - F::cast_from(0.16449340668482264365e-1_f64) * t105453 + F::cast_from(0.24674011002723396548e-1_f64) * t98322 + F::cast_from(12.0_f64) * t4147 * t28307 - t82154 - F::cast_from(3.0_f64) * t17090 * t7538 - t67344 * t1912 + F::cast_from(0.49348022005446793095e-1_f64) * t105462 - F::cast_from(3.0_f64) * t98166 * t1528;
    t105466
}
