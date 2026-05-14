//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1148/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1148<F: Float>(t1464: F, t28382: F, t28499: F, t101943: F, t7968: F, t28721: F, t28778: F, t1489: F, t28503: F, t7282: F, t1497: F, t27387: F, t59071: F, t101938: F, t101941: F, t101944: F, t101948: F, t101950: F, t101954: F, t7971: F, t99035: F) -> (F, F, F, F) {
    let t101957 = t1464 * t28499 * t28382;
    let t101959 = t7968 * t101943;
    let t101961 = t28721 * t28778;
    let t101965 = t1464 * t28503 * t7282 * t1489;
    let t101969 = t1464 * t27387 * t59071 * t1497;
    let t101971 = -0.46429444444444444444e-2 * t101938 + 0.38691203703703703703e-2 * t101941 + t99035 - 0.23168402777777777778e-3 * t101944 - 0.23214722222222222222e-2 * t101948 + 0.33980324074074074074e-2 * t101950 * t7971 - 0.23214722222222222222e-2 * t101954 - 0.77382407407407407407e-3 * t101957 - 0.46377350260416666667e-4 * t101959 + 0.30918233506944444445e-4 * t101961 + 0.34822083333333333332e-2 * t101965 - 0.11607361111111111111e-2 * t101969;
    (t101957, t101965, t101969, t101971)
}
