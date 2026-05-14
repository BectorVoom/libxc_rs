//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1138/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1138<F: Float>(t14443: F, t29006: F, t7703: F, t1003: F, t18482: F, t26686: F, t1008: F, t27806: F, t70767: F, t13097: F, t4977: F, t13376: F, t1704: F, t100179: F, t26685: F, t26748: F, t27775: F, t27780: F, t27832: F, t28984: F, t28988: F, t93366: F, t95686: F) -> (F, F, F, F, F) {
    let t101035 = t7703 * t14443 * t29006;
    let t101043 = t26686 * t18482 * t1003;
    let t101047 = t27806 * t70767 * t1008;
    let t101053 = t27806 * t13097 * t4977;
    let t101057 = t26686 * t13376 * t1704;
    let t101060 = 0.46336805555555555556e-3 * t26748 * t28984 - 0.7722800925925925926e-4 * t101035 - 0.22109259259259259259e-2 * t100179 - 0.27802083333333333334e-2 * t27832 * t27775 - 0.13901041666666666667e-2 * t27832 * t27780 + 0.2782641015625e-3 * t26685 * t101043 + 0.10203017057291666667e-2 * t26685 * t101047 - 0.18550940104166666667e-3 * t93366 * t28988 - 0.556528203125e-3 * t26685 * t101053 - 0.18550940104166666667e-3 * t26685 * t101057 - t95686;
    (t101043, t101047, t101053, t101057, t101060)
}
