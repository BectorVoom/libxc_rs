//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 841/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk841<F: Float>(t181: F, t995: F, t184: F, t2800: F, t2790: F, t2796: F, t1627: F, t3407: F, t1027: F, t2722: F, t1815: F, t639: F, t1044: F, t2705: F, t7199: F, t3469: F, t617: F) -> (F, F, F, F, F, F, F) {
    let t10325 = t995 * t181;
    let t10326 = t10325 * t184;
    let t10328 = 8.0 / 15.0 * t10326 * t2800;
    let t10329 = t2790 * t2796;
    let t10330 = 16.0 / 45.0 * t10329;
    let t10332 = 8.0 / 15.0 * t2790 * t2800;
    let t10334 = 8.0 / 45.0 * t1627 * t3407;
    let t10335 = t1027 * t2722;
    let t10336 = t1815 * t10335;
    let t10338 = 8.0 / 45.0 * t639 * t10336;
    let t10339 = t2705 * t1044;
    let t10340 = t7199 * t10339;
    let t10342 = 16.0 / 45.0 * t639 * t10340;
    let t10343 = t3469 * t617;
    (t10328, t10330, t10332, t10334, t10338, t10342, t10343)
}
