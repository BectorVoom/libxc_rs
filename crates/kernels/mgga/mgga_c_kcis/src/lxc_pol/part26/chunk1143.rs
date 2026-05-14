//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1143/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1143<F: Float>(t22714: F, t7940: F, t27491: F, t7397: F, t28778: F, t28853: F, t28713: F, t6140: F, t27607: F, t28714: F, t28749: F, t28755: F, t28760: F, t28772: F, t29510: F, t7971: F, t8213: F, t98978: F, t98986: F, t98988: F, t99013: F, t99331: F) -> (F, F, F, F) {
    let t101840 = t7940 * t22714;
    let t101841 = t27491 * t7397;
    let t101849 = t28853 * t28778;
    let t101853 = t28713 * t6140;
    let t101862 = 0.34752604166666666667e-3 * t27607 * t29510 + 0.69505208333333333334e-3 * t99013 * t8213 - 0.82448622685185185187e-4 * t101849 + 0.69505208333333333334e-3 * t28714 * t28772 - t98978 - t98986 - t98988 - 0.18534722222222222222e-2 * t101853 * t7971 - 0.61782407407407407408e-3 * t99331 * t28749 - 0.61782407407407407408e-3 * t99331 * t28755 - 0.12356481481481481482e-2 * t99331 * t28760;
    (t101840, t101841, t101853, t101862)
}
