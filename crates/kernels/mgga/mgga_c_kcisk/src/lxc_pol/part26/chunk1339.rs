//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1339/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1339<F: Float>(t1339: F, t26974: F, t32045: F, t33384: F, t33469: F, t33451: F, t1308: F, t3930: F, t8020: F, t1333: F, t34789: F, t110692: F, t110695: F, t113941: F, t114059: F, t114517: F, t119174: F, t19972: F, t27016: F, t2718: F, t32189: F, t33360: F, t34693: F, t9429: F, t9433: F, t9454: F, t9800: F, t9805: F) -> (F, F, F) {
    let t119501 = t1339 * t32045 * t26974;
    let t119505 = t33384 * t33469;
    let t119507 = t33384 * t33451;
    let t119510 = t3930 * t8020 * t1308;
    let t119513 = t1333 * t34789;
    let t119529 = -0.16581944444444444444e-2 * t119501 + 0.18518518518518518519e-1 * t114059 * t9805 - 0.23148148148148148149e-2 * t119505 + 0.69444444444444444447e-2 * t119507 + 0.40208333333333333335e-2 * t119510 * t9429 + 0.33163888888888888888e-2 * t119513 - 0.10416666666666666667e-1 * t27016 * t9433 * t2718 - 0.20833333333333333334e-1 * t19972 * t9800 * t2718 - t114517 + 0.32166666666666666667e-1 * t32189 * t34693 + 0.10416666666666666667e-1 * t119174 * t9454 - 0.23148148148148148149e-2 * t110692 - 0.89351851851851851855e-3 * t110695 + 0.69444444444444444446e-2 * t113941 * t33360;
    (t119501, t119513, t119529)
}
