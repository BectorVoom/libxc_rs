//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1371/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1371<F: Float>(t2326: F, t32440: F, t6204: F, t6587: F, t1163: F, t27965: F, t32464: F, t109499: F, t115027: F, t115036: F, t115500: F, t118745: F, t118748: F, t118789: F, t118792: F, t118795: F, t33802: F, t33823: F, t34934: F, t9536: F, t9855: F, t9860: F) -> (F, F, F) {
    let t120067 = t6204 * t32440 * t2326 * t6587;
    let t120082 = t32464 * t27965 * t1163;
    let t120087 = 0.30952962962962962962e-2 * t118745 + 0.77382407407407407407e-3 * t118748 - 0.23148148148148148148e-2 * t115027 - 0.10416666666666666667e-1 * t9536 * t120067 - 0.27777777777777777779e-1 * t33802 * t9855 - 0.41270617283950617283e-2 * t118789 - 0.19345601851851851852e-2 * t118792 + 0.12897067901234567901e-2 * t118795 + 0.10416666666666666667e-1 * t9860 * t33823 - 0.34722222222222222223e-2 * t9536 * t109499 * t34934 * t1163 + 0.17361111111111111111e-2 * t9536 * t120082 - t115036 - 0.10722222222222222222e-1 * t115500 * t9855;
    (t120067, t120082, t120087)
}
