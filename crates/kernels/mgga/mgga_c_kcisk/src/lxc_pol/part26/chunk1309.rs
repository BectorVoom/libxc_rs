//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1309/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1309<F: Float>(t20160: F, t34696: F, t9446: F, t1327: F, t32069: F, t6204: F, t8054: F, t113641: F, t113643: F, t113650: F, t113666: F, t114674: F, t1163: F, t118781: F, t118785: F, t118789: F, t118792: F, t118795: F, t118797: F, t32008: F, t32013: F, t32087: F, t33360: F, t3937: F, t81032: F) -> (F, F, F) {
    let t118802 = t20160 * t34696;
    let t118803 = t9446 * t118802;
    let t118812 = t6204 * t32069 * t8054 * t1327;
    let t118815 = 0.26805555555555555556e-2 * t114674 * t33360 + 0.53611111111111111112e-2 * t32008 * t118781 + 0.13402777777777777778e-2 * t32008 * t118785 - 0.58958024691358024688e-2 * t118789 - 0.27636574074074074073e-2 * t118792 + 0.18424382716049382715e-2 * t118795 - t113641 - t113643 - t113650 + 0.34722222222222222223e-2 * t32087 * t3937 * t118797 * t1163 + 0.34722222222222222223e-2 * t118803 - 0.7369753086419753086e-3 * t113666 - 0.20833333333333333334e-1 * t9446 * t6204 * t32013 * t81032 - 0.10416666666666666667e-1 * t9446 * t118812;
    (t118802, t118812, t118815)
}
