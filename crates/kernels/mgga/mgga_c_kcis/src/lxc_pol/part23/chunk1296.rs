//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1296/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1296<F: Float>(t27607: F, t28781: F, t18114: F, t27560: F, t28721: F, t28749: F, t28755: F, t28760: F, t6176: F, t7978: F, t7984: F, t94340: F, t94928: F, t94931: F, t98313: F, t98328: F, t98331: F) -> F {
    let t99260 = F::cast_from(0.23168402777777777778e-3_f64) * t27607 * t28781;
    let t99276 = F::cast_from(0.15476481481481481481e-2_f64) * t94340 + F::cast_from(0.46377350260416666667e-4_f64) * t28721 * t27560 + t99260 + F::cast_from(0.34752604166666666667e-3_f64) * t7978 * t6176 * t7984 * t18114 - F::new(0.10446625e-1) * t98313 + F::cast_from(0.23168402777777777778e-3_f64) * t94928 * t28749 + F::cast_from(0.23168402777777777778e-3_f64) * t94928 * t28755 + F::cast_from(0.46336805555555555556e-3_f64) * t94928 * t28760 + F::cast_from(0.30918233506944444444e-4_f64) * t94931 * t28755 - F::cast_from(0.46429444444444444444e-2_f64) * t98328 - F::cast_from(0.23214722222222222222e-2_f64) * t98331;
    t99276
}
