//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1376/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1376<F: Float>(t33778: F, t33873: F, t1586: F, t2738: F, t84866: F, t2737: F, t34989: F, t4419: F, t1163: F, t27958: F, t32464: F, t113855: F, t115213: F, t115215: F, t115240: F, t115247: F, t115251: F, t115253: F, t118944: F, t34945: F, t34990: F, t9516: F, t9529: F, t9536: F) -> (F, F, F) {
    let t120204 = t33778 * t33873;
    let t120207 = t1586 * t2738 * t84866;
    let t120213 = t2737 * t4419 * t34989;
    let t120220 = t32464 * t27958 * t1163;
    let t120223 = 0.27777777777777777779e-1 * t9529 * t34945 + 0.13402777777777777778e-2 * t120204 + 0.20104166666666666667e-2 * t9516 * t120207 + t115213 + t115215 - 0.13888888888888888889e-1 * t9529 * t34990 + 0.17361111111111111111e-2 * t120213 - 0.35740740740740740741e-2 * t115240 + 0.38691203703703703703e-2 * t118944 - 0.89351851851851851853e-3 * t115247 - t115251 + 0.46429444444444444444e-2 * t113855 - t115253 - 0.34722222222222222223e-2 * t9536 * t120220;
    (t120207, t120220, t120223)
}
