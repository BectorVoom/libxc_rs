//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1296/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1296(t27607: f64, t28781: f64, t18114: f64, t27560: f64, t28721: f64, t28749: f64, t28755: f64, t28760: f64, t6176: f64, t7978: f64, t7984: f64, t94340: f64, t94928: f64, t94931: f64, t98313: f64, t98328: f64, t98331: f64) -> f64 {
    let t99260 = 0.23168402777777777778e-3_f64 * t27607 * t28781;
    let t99276 = 0.15476481481481481481e-2_f64 * t94340 + 0.46377350260416666667e-4_f64 * t28721 * t27560 + t99260 + 0.34752604166666666667e-3_f64 * t7978 * t6176 * t7984 * t18114 - 0.10446625e-1_f64 * t98313 + 0.23168402777777777778e-3_f64 * t94928 * t28749 + 0.23168402777777777778e-3_f64 * t94928 * t28755 + 0.46336805555555555556e-3_f64 * t94928 * t28760 + 0.30918233506944444444e-4_f64 * t94931 * t28755 - 0.46429444444444444444e-2_f64 * t98328 - 0.23214722222222222222e-2_f64 * t98331;
    t99276
}
