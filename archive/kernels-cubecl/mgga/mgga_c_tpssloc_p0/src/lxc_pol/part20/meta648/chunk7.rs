//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2386/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2386<F: Float>(t48960: F, t48980: F, t901: F, t41662: F, t41675: F, t41678: F, t41682: F, t41684: F, t41863: F, t41865: F, t41870: F, t41872: F, t41874: F, t41876: F) -> (F, F, F) {
    let t48981 = t48960 + t48980;
    let t48982 = t901 * t48981;
    let t48990 = F::cast_from(0.99655555555555555557e-1_f64) * t41662 + F::cast_from(0.79724444444444444447e0_f64) * t41675 - F::cast_from(0.39862222222222222222e0_f64) * t41678 + F::cast_from(0.59793333333333333333e0_f64) * t41682 + F::cast_from(0.93011851851851851855e0_f64) * t41684 + F::cast_from(0.3071625e0_f64) * t48982 + F::cast_from(0.73028148148148148149e0_f64) * t41863 - F::cast_from(0.10954222222222222222e0_f64) * t41865 - F::cast_from(0.27385555555555555556e0_f64) * t41870 - F::cast_from(0.91285185185185185185e-1_f64) * t41872 + F::cast_from(0.54771111111111111111e-1_f64) * t41874 + F::cast_from(0.24342716049382716049e-1_f64) * t41876;
    (t48981, t48982, t48990)
}
