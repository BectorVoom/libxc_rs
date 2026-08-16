//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1035/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1035<F: Float>(t114335: F, t22574: F, t24432: F, t112547: F, t115721: F, t115725: F, t115727: F, t115728: F, t115732: F, t115738: F, t115743: F, t115748: F, t115750: F, t115752: F, t115754: F, t1266: F, t1393: F, t2096: F, t23958: F, t24028: F, t31246: F, t31700: F, t31722: F, t7218: F, t8450: F) -> F {
    let t115757 = F::cast_from(6.0_f64) * t22574 * t24432 * t114335;
    let t115758 = t112547 * t2096 - F::cast_from(2.0_f64) * t1266 * t31700 + F::cast_from(2.0_f64) * t1393 * t31722 + F::cast_from(6.0_f64) * t23958 * t8450 - F::cast_from(2.0_f64) * t24028 * t8450 + F::cast_from(2.0_f64) * t31246 * t7218 + t115721 - t115725 - t115727 - t115728 - t115732 - t115738 - t115743 - t115748 + t115750 - t115752 - t115754 - t115757;
    t115758
}
