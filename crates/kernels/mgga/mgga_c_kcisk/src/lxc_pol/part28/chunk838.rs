//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 838/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk838<F: Float>(t1884: F, t3517: F, t10671: F, t677: F, t1821: F, t4663: F, t10568: F, t1797: F, t180: F, t479: F, t574: F, t682: F, t695: F, t10459: F, t707: F, t213: F, t568: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11350 = t3517 * t1884;
    let t11352 = t10671 * t677;
    let t11355 = t4663 * t1821;
    let t11371 = 0.12841111111111111111e-1 * t10568;
    let t11400 = t180 * t479 * t1797;
    let t11401 = t574 * t682;
    let t11402 = t11401 * t695;
    let t11417 = t10459 * t707;
    let t11458 = t213 * t568;
    (t11350, t11352, t11355, t11371, t11400, t11401, t11402, t11417, t11458)
}
