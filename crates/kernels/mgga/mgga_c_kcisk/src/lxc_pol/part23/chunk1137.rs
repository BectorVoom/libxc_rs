//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1137/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1137<F: Float>(t2715: F, t32105: F, t9434: F, t9442: F, t20: F, t388: F, t3913: F, t1220: F, t9439: F, t2718: F, t32010: F, t32035: F, t32060: F, t32063: F, t32066: F, t32072: F, t32076: F, t32079: F, t32082: F, t32084: F, t32087: F, t32090: F, t32096: F, t32102: F, t9426: F, t9429: F, t9454: F) -> (F, F, F, F, F, F) {
    let t32107 = 0.23148148148148148149e-2 * t2715 * t32105;
    let t32108 = t9434 * t9442;
    let t32111 = t388 * t3913 * t20;
    let t32112 = t1220 * t32111;
    let t32115 = t9439 * t9442;
    let t32117 = -0.49745833333333333332e-2 * t32060 + 0.33163888888888888888e-2 * t32063 + 0.8041666666666666667e-2 * t32066 * t9429 - 0.8041666666666666667e-2 * t9426 * t32072 + 0.13265555555555555555e-1 * t32076 - 0.88437037037037037034e-2 * t32079 + 0.16581944444444444444e-2 * t32082 - 0.20833333333333333334e-1 * t32084 * t2718 + 0.69444444444444444446e-2 * t32087 * t32090 + 0.69444444444444444446e-2 * t32087 * t32010 + 0.20833333333333333334e-1 * t32096 * t9454 + 0.20833333333333333334e-1 * t32096 * t9429 - 0.23280625000000000001e-2 * t32102 * t32035 + t32107 - 0.69444444444444444446e-2 * t32108 - 0.10185185185185185186e0 * t32112 * t2718 + 0.18518518518518518519e-1 * t32115;
    (t32107, t32108, t32111, t32112, t32115, t32117)
}
