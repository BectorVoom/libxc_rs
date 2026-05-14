//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 592/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk592<F: Float>(t1546: F, t1979: F, t89: F, t1965: F, t7780: F, t1987: F, t375: F, t128: F, t39: F, t2035: F, t2044: F, t7853: F, t548: F, t8078: F, t120: F, t1655: F) -> (F, F, F, F, F, F, F, F) {
    let t8799 = t89 * t1546 * t1979;
    let t8802 = t89 * t7780 * t1965;
    let t8805 = t89 * t375 * t1987;
    let t8811 = t128 * t39;
    let t8812 = t8811 * t2035;
    let t8885 = t7853 * t2044;
    let t8906 = t548 * t548;
    let t8907 = 1.0 / t8906;
    let t8914 = 0.18521666970164609055e-1 * t8078;
    let t8942 = t120 * t1655;
    (t8799, t8802, t8805, t8812, t8885, t8907, t8914, t8942)
}
