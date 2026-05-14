//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1139/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1139<F: Float>(t2804: F, t33234: F, t4644: F, t9741: F, t1775: F, t32899: F, t32901: F, t32905: F, t32925: F, t32928: F, t33180: F, t33196: F, t33200: F, t33212: F, t33222: F, t33229: F, t9725: F, t9740: F) -> (F, F, F, F) {
    let t33235 = t2804 * t33234;
    let t33239 = t9741 * t4644;
    let t33240 = t1775 * t33239;
    let t33245 = -t33212 + 0.17411041666666666666e-2 * t32899 + 0.15476481481481481481e-2 * t32901 + 0.23214722222222222222e-2 * t32905 - 0.60312500000000000001e-2 * t9725 * t33180 + 0.34722222222222222222e-2 * t9740 * t33222 + 0.34722222222222222222e-2 * t9740 * t33229 + 0.13402777777777777778e-2 * t33196 * t33229 + 0.34722222222222222222e-2 * t33235 + 0.15476481481481481481e-2 * t32925 + 0.11607361111111111111e-2 * t32928 + 0.34722222222222222222e-2 * t9740 * t33240 - 0.10416666666666666667e-1 * t9740 * t33200;
    (t33235, t33239, t33240, t33245)
}
