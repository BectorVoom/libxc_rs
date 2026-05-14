//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 863/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk863<F: Float>(t31805: F, t1198: F, t2095: F, t355: F, t151: F, t7731: F, t950: F, t947: F, t7685: F, t932: F, t2016: F, t7592: F, t3378: F, t7560: F, t1177: F, t13364: F, t31115: F, t31116: F) -> (F, F, F, F, F, F, F, F) {
    let t31806 = 0.64311027177104605458e-3 * t31805;
    let t31808 = t2095 * t1198 * t355;
    let t31811 = t151 * t7731 * t950;
    let t31812 = t31811 * t947;
    let t31816 = t7685 * t932;
    let t31822 = t2016 * t7592;
    let t31824 = t3378 * t7560;
    let t31825 = t31824 * t1177;
    let t31832 = t31115 * t13364 * t31116;
    (t31806, t31808, t31812, t31816, t31822, t31824, t31825, t31832)
}
