//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1043/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1043<F: Float>(t1860: F, t2032: F, t22493: F, t22519: F, t22527: F, t22531: F, t22534: F, t22537: F, t22546: F, t22549: F, t23963: F, t23968: F, t23970: F, t23973: F, t23975: F, t23978: F, t23995: F, t23999: F, t24001: F, t6486: F, t6492: F, t6495: F, t7026: F, t7035: F) -> F {
    let t24006 = F::cast_from(10.0_f64) * t23963 * t22546 + F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t23968 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t22549 * t23970 + F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t23973 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t23975 * t6492 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t23978 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t22519 * t2032 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t7026 * t22527 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t7026 * t22531 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t22534 * t2032 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t22537 * t2032 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t6495 * t7035 + t23995 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t6486 * t7035 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t23999 + t1860 * t24001 / F::cast_from(3.0_f64) + t22493 * t2032 / F::cast_from(3.0_f64);
    t24006
}
