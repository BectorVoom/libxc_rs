//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1354/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1354<F: Float>(t112773: F, t112776: F, t112780: F, t112791: F, t116181: F, t116184: F, t116194: F, t117654: F, t117663: F, t117668: F, t117674: F, t117683: F, t33196: F, t33297: F, t34534: F, t9728: F, t9740: F) -> (F,) {
    let t117686 = 0.20833333333333333334e-1 * t9740 * t117654 - 0.34722222222222222222e-2 * t112773 + 0.17361111111111111111e-2 * t112776 + 0.12897067901234567901e-2 * t116181 + 0.46429444444444444444e-2 * t116184 - 0.10416666666666666667e-1 * t9740 * t117663 - 0.20833333333333333334e-1 * t9740 * t117668 + 0.22114583333333333334e-1 * t33196 * t117674 - 0.23148148148148148148e-2 * t112780 + 0.34722222222222222222e-2 * t33297 * t34534 + 0.17361111111111111111e-2 * t112791 - 0.19345601851851851852e-2 * t116194 + 0.40208333333333333334e-2 * t117683 * t9728;
    (t117686,)
}
