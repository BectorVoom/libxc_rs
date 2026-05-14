//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1386/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1386<F: Float>(t10012: F, t112724: F, t112726: F, t112739: F, t112867: F, t117237: F, t117246: F, t117621: F, t117668: F, t118120: F, t118275: F, t33200: F, t33297: F, t34406: F, t34412: F, t34416: F, t34419: F, t34435: F, t4640: F, t9740: F) -> (F,) {
    let t118532 = 0.23214722222222222222e-2 * t112724 - 0.17411041666666666666e-2 * t117237 - 0.25794135802469135802e-3 * t112726 + 0.69644166666666666664e-2 * t117246 + 0.23148148148148148148e-2 * t9740 * t112867 * t10012 * t4640 - 0.23214722222222222222e-2 * t112739 + 0.27777777777777777778e-1 * t34412 * t33200 + 0.10722222222222222222e-1 * t118120 * t33200 - 0.10416666666666666667e-1 * t34416 * t33200 - 0.40208333333333333334e-2 * t118275 * t33200 - 0.23280625e-2 * t34419 * t117668 - 0.20833333333333333334e-1 * t33297 * t34406 - 0.10416666666666666667e-1 * t34435 * t33200 - 0.40208333333333333334e-2 * t117621 * t33200;
    (t118532,)
}
