//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1780/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1780<F: Float>(t3917: F, t47506: F, t2453: F, t3908: F, t4067: F, t10115: F, t1421: F, t10168: F, t3920: F, t10147: F, t4071: F, t47472: F, t47474: F, t47478: F, t47483: F, t47487: F, t47490: F, t47493: F, t47495: F, t47497: F, t47500: F, t47504: F) -> F {
    let t47507 = t47506 * t3917;
    let t47510 = t2453 * t4067 * t3908;
    let t47512 = t10115 * t1421;
    let t47516 = t10168 * t3920;
    let t47518 = -F::cast_from(0.13170898365871023197e0_f64) * t47472 + F::cast_from(0.12142592671231907757e0_f64) * t47474 - F::cast_from(0.12142592671231907757e0_f64) * t47478 + F::cast_from(0.13878983423218070567e-1_f64) * t47483 + F::cast_from(0.18505311230957427423e-1_f64) * t47487 + F::cast_from(0.39029762157531132076e-1_f64) * t47490 - F::cast_from(0.13170898365871023197e0_f64) * t47493 - F::cast_from(0.1040793657534163522e-1_f64) * t47495 + F::cast_from(0.68293547082294194357e-1_f64) * t47497 + F::cast_from(0.7805952431506226415e-2_f64) * t47500 + t47504 - F::cast_from(0.11708928647259339623e0_f64) * t47507 + F::cast_from(0.69394917116090352835e-2_f64) * t47510 - F::cast_from(0.44178176337912614788e-3_f64) * t47512 - F::cast_from(0.26341796731742046395e1_f64) * t4071 * t10147 - F::cast_from(0.78059524315062264152e-1_f64) * t47516;
    t47518
}
