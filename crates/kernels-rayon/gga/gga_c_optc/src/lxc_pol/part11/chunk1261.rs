//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1261/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1261(t25085: f64, t55901: f64, t894: f64, t14330: f64, t16917: f64, t24535: f64, t56726: f64, t10991: f64, t14300: f64, t14329: f64, t14339: f64, t14360: f64, t16968: f64, t24546: f64, t24601: f64, t2640: f64, t41484: f64, t41498: f64, t41526: f64, t50941: f64, t51027: f64, t51035: f64, t7449: f64, t7491: f64, t893: f64) -> (f64, f64, f64, f64) {
    let t56867 = t894 * t25085 * t55901;
    let t56877 = t14330 * t16917;
    let t56881 = t24535 * t56726;
    let t56891 = -0.43465807448943789272e-1_f64 * t893 * t56867 - 0.36629113921839320676e2_f64 * t7449 * t50941 * t14339 - 0.47333755318775392234e-1_f64 * t41484 + t41498 / 108.0_f64 - t24546 - 0.36629113921839320676e2_f64 * t51035 - t24601 - 0.63111673758367189645e-1_f64 * t41526 - 0.5680050638253047068e0_f64 * t10991 * t14329 * t56877 - 0.73258227843678641351e2_f64 * t7491 * t14300 * t56881 + 0.36629113921839320675e2_f64 * t7449 * t14300 * t51027 + 0.1420012659563261767e0_f64 * t2640 * t14360 * t16968;
    (t56867, t56877, t56881, t56891)
}
