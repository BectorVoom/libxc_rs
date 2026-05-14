//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 978/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk978<F: Float>(t10570: F, t10615: F, t10639: F, t10642: F, t15989: F, t15993: F, t15996: F, t16389: F, t16392: F, t16400: F, t16485: F, t16500: F, t22575: F, t22583: F, t22610: F, t22684: F, t22701: F, t22705: F, t22707: F, t22711: F, t22714: F, t22743: F) -> (F,) {
    let t22745 = -0.26837777777777777779e0 * t15989 - 0.40256666666666666668e0 * t15996 - 0.91983333333333333333e-1 * t10615 - 0.13418888888888888889e0 * t10570 + t16485 - 0.40256666666666666668e0 * t15993 - t10639 - 0.18396666666666666667e0 * t16389 - 0.22076e0 * t16392 + 0.16557e0 * t22684 + t22701 - 0.20128333333333333333e0 * t22575 + 0.10064166666666666667e0 * t22583 - 0.11038e0 * t22705 + 0.5519e-1 * t22707 - t16500 + 0.36793333333333333333e-1 * t16400 - t10642 + 0.258925e1 * t22610 + 0.16504875e0 * t22711 - 0.36793333333333333333e-1 * t22714 + t22743;
    (t22745,)
}
