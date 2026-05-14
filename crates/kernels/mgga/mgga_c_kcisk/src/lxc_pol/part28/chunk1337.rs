//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1337/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1337<F: Float>(t112761: F, t34500: F, t9740: F, t34495: F, t33196: F, t25: F, t34559: F, t34562: F, t10000: F, t33276: F, t2804: F, t34519: F, t4419: F, t34468: F, t9725: F, t116380: F) -> (F, F, F, F, F, F, F, F, F) {
    let t117873 = 0.23148148148148148148e-2 * t9740 * t112761 * t34500;
    let t117874 = t112761 * t34495;
    let t117876 = 0.44675925925925925926e-3 * t33196 * t117874;
    let t117880 = 0.15432098765432098765e-2 * t9740 * t25 * t34559 * t34562;
    let t117887 = t10000 * t33276;
    let t117897 = 0.34722222222222222222e-2 * t2804 * t4419 * t34519;
    let t117898 = t4419 * t34468;
    let t117900 = 0.13402777777777777778e-2 * t9725 * t117898;
    let t117906 = 0.15476481481481481481e-2 * t116380;
    (t117873, t117874, t117876, t117880, t117887, t117897, t117898, t117900, t117906)
}
