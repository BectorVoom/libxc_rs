//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1315/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1315<F: Float>(t1327: F, t32033: F, t6204: F, t8059: F, t2168: F, t32069: F, t6217: F, t33604: F, t3759: F, t5635: F, t33357: F, t6183: F, t7710: F, t113800: F, t113805: F, t113854: F, t113856: F, t113857: F, t114199: F, t114531: F, t118741: F, t118781: F, t26045: F, t32008: F, t32087: F, t32088: F, t32102: F, t3937: F, t88147: F, t9426: F, t9796: F) -> (F, F, F, F, F) {
    let t118933 = t6204 * t32033 * t8059 * t1327;
    let t118938 = t6204 * t32069 * t6217 * t2168;
    let t118944 = t3759 * t33604 * t5635;
    let t118962 = t6183 * t33357 * t7710 * t1327;
    let t118965 = -0.23280625000000000001e-2 * t32102 * t118933 - 0.8041666666666666667e-2 * t9426 * t118938 + 0.89351851851851851851e-3 * t113800 + 0.23148148148148148149e-2 * t113805 + 0.55273148148148148147e-2 * t118944 - 0.26805555555555555556e-2 * t32008 * t118741 + 0.92592592592592592594e-2 * t32087 * t114531 * t26045 + 0.20833333333333333334e-1 * t114199 * t9796 - t113854 + t113856 - 0.22109259259259259259e-2 * t113857 + 0.13888888888888888889e-1 * t32087 * t118781 + 0.34722222222222222223e-2 * t32087 * t3937 * t32088 * t88147 + 0.34722222222222222223e-2 * t32087 * t118962;
    (t118933, t118938, t118944, t118962, t118965)
}
