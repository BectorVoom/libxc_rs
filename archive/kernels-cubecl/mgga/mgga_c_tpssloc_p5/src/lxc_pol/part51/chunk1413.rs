//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1413/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1413<F: Float>(t114992: F, t118413: F, t118953: F, t121774: F, t121779: F, t1530: F, t1877: F, t1914: F, t193: F, t202: F, t24344: F, t2522: F, t25353: F, t25365: F, t25374: F, t26744: F, t26756: F, t31434: F, t31448: F, t33466: F, t33483: F, t4119: F, t4255: F, t4314: F, t6665: F, t7114: F, t776: F, t82312: F, t84800: F, t8566: F, t870: F, t92276: F, t93000: F) -> F {
    let t121949 = F::cast_from(2.0_f64) * t1877 * t93000 * t31448 - F::cast_from(6.0_f64) * t26756 * t82312 * t25374 + F::cast_from(3.0_f64) * t2522 * t8566 * t4119 - F::cast_from(3.0_f64) * t2522 * t31434 * t25365 + t193 * t202 * t121774 * t870 - t1877 * t92276 * t1914 + F::cast_from(3.0_f64) * t2522 * t33466 * t776 + F::cast_from(6.0_f64) * t4314 * t8566 * t4255 - t1877 * t7114 * t25353 - t1877 * t114992 * t1530 - t1877 * t26744 * t6665 + F::cast_from(2.0_f64) * t1877 * t24344 * t118953 + F::cast_from(2.0_f64) * t1877 * t24344 * t118413 + F::cast_from(2.0_f64) * t1877 * t24344 * t121779 + F::cast_from(2.0_f64) * t1877 * t84800 * t33483;
    t121949
}
