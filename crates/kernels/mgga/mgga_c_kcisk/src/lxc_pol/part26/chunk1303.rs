//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1303/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1303<F: Float>(t109321: F, t27057: F, t34816: F, t3748: F, t26495: F, t6204: F, t9427: F, t1411: F, t19848: F, t33609: F, t394: F, t110605: F, t1163: F, t34742: F, t113578: F, t113579: F, t113582: F, t113584: F, t32019: F, t32022: F, t32026: F, t32066: F, t32087: F, t32096: F, t34693: F, t34749: F, t9446: F) -> (F, F, F, F, F, F) {
    let t118682 = 6.0 * t109321 * t27057;
    let t118689 = t3748 * t34816;
    let t118699 = t6204 * t9427 * t26495;
    let t118704 = t1411 * t19848 * t394 * t33609;
    let t118707 = t110605 * t34742 * t1163;
    let t118710 = 0.55555555555555555557e-1 * t32022 * t34749 + t113578 - 0.120625e-1 * t32026 * t34693 - 0.120625e-1 * t32066 * t34693 + 0.11054629629629629629e-2 * t118689 - 0.20833333333333333334e-1 * t32096 * t34749 - 0.20833333333333333334e-1 * t32019 * t34749 - 0.71481481481481481483e-2 * t113579 - 0.17870370370370370371e-2 * t113582 - 0.58958024691358024688e-2 * t113584 + 0.10416666666666666667e-1 * t9446 * t118699 + 0.99491666666666666664e-2 * t118704 - 0.69444444444444444447e-2 * t32087 * t118707;
    (t118682, t118689, t118699, t118704, t118707, t118710)
}
