//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1262/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1262<F: Float>(t7884: F, t9858: F, t1230: F, t125: F, t19557: F, t19568: F, t19571: F, t19574: F, t19577: F, t19579: F, t19664: F, t22991: F, t22994: F, t22997: F, t27007: F, t27015: F, t27018: F, t27021: F, t27023: F, t27025: F, t2986: F, t555: F, t557: F) -> F {
    let t27027 = t7884 * t9858;
    let t27034 = -F::cast_from(5.0_f64) / F::cast_from(432.0_f64) * t22991 + t22994 / F::cast_from(72.0_f64) + t22997 / F::cast_from(72.0_f64) + t27007 / F::cast_from(288.0_f64) + t19557 + t19568 / F::cast_from(48.0_f64) - F::cast_from(5.0_f64) / F::cast_from(144.0_f64) * t19571 - F::cast_from(5.0_f64) / F::cast_from(144.0_f64) * t19574 + t19577 / F::cast_from(96.0_f64) - F::cast_from(5.0_f64) / F::cast_from(144.0_f64) * t19579 + t19664 / F::cast_from(48.0_f64) - t27015 / F::cast_from(32.0_f64) - t27018 / F::cast_from(32.0_f64) - t27021 / F::cast_from(32.0_f64) - t27023 / F::cast_from(16.0_f64) - t27025 / F::cast_from(16.0_f64) + F::cast_from(7.0_f64) / F::cast_from(16.0_f64) * t27027 - t555 * t2986 * t557 * t1230 * t125 / F::cast_from(16.0_f64);
    t27034
}
