//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 931/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk931<F: Float>(t10642: F, t16032: F, t16410: F, t16413: F, t16416: F, t16419: F, t16421: F, t16424: F, t16427: F, t16432: F, t16434: F, t10570: F, t10607: F, t10615: F, t10617: F, t10619: F, t10639: F, t15989: F, t15996: F, t16028: F, t16045: F, t16048: F, t16068: F, t16070: F, t16389: F, t16392: F, t16396: F, t16400: F, t16403: F, t16406: F, t16493: F, t16500: F) -> (F,) {
    let t16515 = 0.11038e0 * t16410 - t10642 + 0.16557e0 * t16413 - 0.49671e0 * t16416 + 0.16504875e0 * t16419 + 0.82524375e-1 * t16421 + 0.16557e0 * t16424 - 0.66228e0 * t16427 + 0.60385e0 * t16032 + 0.16557e0 * t16432 + 0.16504875e0 * t16434;
    let t16517 = 0.258925e1 * t16045 - 0.258925e1 * t16068 - 0.1294625e1 * t16070 - 0.13418888888888888889e0 * t15989 - 0.22141166666666666666e1 * t15996 - 0.11038e0 * t10607 - 0.18396666666666666667e0 * t10615 + 0.5519e-1 * t10617 + 0.18396666666666666667e-1 * t10619 - 0.26837777777777777778e0 * t10570 + t16493 + 0.60385e0 * t16028 - t10639 - 0.91983333333333333334e-1 * t16389 - 0.71747e0 * t16392 + 0.19419375e1 * t16048 - 0.412621875e-1 * t16396 - t16500 + 0.36793333333333333334e-1 * t16400 - 0.27595e-1 * t16403 - 0.36793333333333333333e-1 * t16406 + t16515;
    (t16517,)
}
