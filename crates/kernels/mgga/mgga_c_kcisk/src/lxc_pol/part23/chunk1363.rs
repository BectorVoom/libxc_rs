//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1363/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1363<F: Float>(t33372: F, t3969: F, t32173: F, t33373: F, t13485: F, t32087: F, t33445: F, t32176: F, t3783: F, t394: F, t470: F, t18998: F, t3482: F, t110440: F, t110443: F, t110445: F, t13472: F, t19033: F, t20188: F, t2059: F, t32035: F, t32066: F, t32088: F, t32180: F, t33346: F, t33415: F, t3961: F, t442: F, t6183: F, t6187: F, t6204: F, t9446: F, t9447: F, t9452: F, t9454: F) -> (F, F, F) {
    let t113997 = t33372 * t3969;
    let t114001 = 0.69444444444444444446e-2 * t33373 * t32173;
    let t114004 = 0.23148148148148148148e-2 * t32087 * t13485 * t33445;
    let t114011 = 0.69444444444444444446e-2 * t33373 * t32176;
    let t114021 = t3783 * t394 * t470;
    let t114023 = t3482 * t114021 * t18998;
    let t114025 = 0.8041666666666666667e-2 * t32066 * t33346 + 0.34722222222222222223e-2 * t110440 + 0.10416666666666666667e-1 * t9446 * t6204 * t9452 * t20188 + 0.10416666666666666667e-1 * t33373 * t32180 - 0.69444444444444444446e-2 * t9446 * t6187 * t9447 * t442 - 0.55555555555555555558e-1 * t113997 * t9454 + t114001 + t114004 - 0.69444444444444444446e-2 * t32087 * t13472 * t32088 * t2059 * t3961 + t114011 - 0.20833333333333333334e-1 * t33373 * t32035 + 0.26805555555555555556e-2 * t110443 - 0.77602083333333333335e-3 * t110445 - 0.20833333333333333334e-1 * t32087 * t6183 * t33415 * t19033 + 0.66327777777777777776e-2 * t114023;
    (t113997, t114023, t114025)
}
