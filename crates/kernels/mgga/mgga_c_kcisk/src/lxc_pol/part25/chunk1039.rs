//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1039/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1039<F: Float>(t16398: F, t12043: F, t16032: F, t16410: F, t16413: F, t16416: F, t16419: F, t16421: F, t16424: F, t16427: F, t16432: F, t16434: F, t10570: F, t10607: F, t10615: F, t10617: F, t10619: F, t12042: F, t15989: F, t15996: F, t16028: F, t16045: F, t16048: F, t16068: F, t16070: F, t16389: F, t16392: F, t16396: F, t16400: F, t16403: F, t16406: F, t18507: F) -> (F,) {
    let t18514 = 0.27785333333333333334e0 * t16398;
    let t18529 = 0.13892666666666666667e0 * t16410 - t12043 + 0.20839e0 * t16413 - 0.62517e0 * t16416 + 0.6311625e0 * t16419 + 0.31558125e0 * t16421 + 0.20839e0 * t16424 - 0.83356e0 * t16427 + 0.103295e1 * t16032 + 0.20839e0 * t16432 + 0.6311625e0 * t16434;
    let t18531 = 0.3529725e1 * t16045 - 0.3529725e1 * t16068 - 0.17648625e1 * t16070 - 0.22954444444444444444e0 * t15989 - 0.37874833333333333334e1 * t15996 - 0.13892666666666666667e0 * t10607 - 0.23154444444444444444e0 * t10615 + 0.69463333333333333333e-1 * t10617 + 0.23154444444444444444e-1 * t10619 - 0.45908888888888888888e0 * t10570 + t18507 + 0.103295e1 * t16028 - t12042 - 0.11577222222222222222e0 * t16389 - 0.90302333333333333334e0 * t16392 + 0.264729375e1 * t16048 - 0.157790625e0 * t16396 - t18514 + 0.46308888888888888889e-1 * t16400 - 0.34731666666666666667e-1 * t16403 - 0.46308888888888888889e-1 * t16406 + t18529;
    (t18531,)
}
