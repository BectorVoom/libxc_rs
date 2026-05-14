//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 980/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk980<F: Float>(t13873: F, t13902: F, t20226: F, t20255: F, t26026: F, t26079: F, t26445: F, t26447: F, t26450: F, t26455: F, t26460: F, t26492: F, t3935: F, t6176: F, t6180: F, t6184: F) -> (F,) {
    let t26495 = t26026 + t26079 + t26445 + t13873 - 0.3997953048039403825e-2 * t13902 - t20226 - 0.35981577432354634426e-1 * t3935 * t26447 + 0.23987718288236422951e-1 * t3935 * t26450 + 0.35981577432354634427e-1 * t3935 * t26455 - 0.17990788716177317213e-1 * t3935 * t26460 - 0.35981577432354634426e-1 * t20255 * t6180 - 0.71963154864709268852e-1 * t20255 * t6184 + 0.47975436576472845901e-1 * t20255 * t6176 + t26492;
    (t26495,)
}
