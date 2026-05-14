//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1134/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1134<F: Float>(t25085: F, t55901: F, t894: F, t14330: F, t16917: F, t24535: F, t56726: F, t10991: F, t14300: F, t14329: F, t14339: F, t14360: F, t16968: F, t24546: F, t24601: F, t2640: F, t41484: F, t41498: F, t41526: F, t50941: F, t51027: F, t51035: F, t7449: F, t7491: F, t893: F) -> (F, F, F, F) {
    let t56867 = t894 * t25085 * t55901;
    let t56877 = t14330 * t16917;
    let t56881 = t24535 * t56726;
    let t56891 = -0.43465807448943789272e-1 * t893 * t56867 - 0.36629113921839320676e2 * t7449 * t50941 * t14339 - 0.47333755318775392234e-1 * t41484 + t41498 / 108.0 - t24546 - 0.36629113921839320676e2 * t51035 - t24601 - 0.63111673758367189645e-1 * t41526 - 0.5680050638253047068e0 * t10991 * t14329 * t56877 - 0.73258227843678641351e2 * t7491 * t14300 * t56881 + 0.36629113921839320675e2 * t7449 * t14300 * t51027 + 0.1420012659563261767e0 * t2640 * t14360 * t16968;
    (t56867, t56877, t56881, t56891)
}
