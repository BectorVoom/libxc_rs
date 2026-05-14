//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 940/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk940<F: Float>(t1869: F, t22311: F, t15866: F, t15871: F, t22252: F, t22256: F, t22260: F, t22263: F, t22265: F, t22269: F, t22272: F, t22275: F, t22281: F, t22286: F, t22292: F, t22297: F, t22299: F, t22301: F, t22303: F, t22305: F, t22308: F) -> (F, F) {
    let t22312 = t1869 * t22311;
    let t22314 = -0.24872916666666666666e-2 * t22252 + 0.16581944444444444444e-2 * t22256 + 0.17687407407407407407e-1 * t22260 - 0.33163888888888888888e-2 * t22263 + 0.1621345679012345679e-1 * t22265 - 0.22109259259259259258e-2 * t22269 - 0.24872916666666666666e-2 * t22272 + 0.16581944444444444444e-2 * t22275 - 0.58958024691358024689e-2 * t15866 + 0.33163888888888888888e-2 * t22281 + 0.99491666666666666664e-2 * t22286 - 0.44218518518518518516e-2 * t15871 + 0.99491666666666666664e-2 * t22292 + 0.13265555555555555555e-1 * t22297 - 0.33163888888888888888e-2 * t22299 - 0.33163888888888888888e-2 * t22301 - 0.33163888888888888888e-2 * t22303 + 0.22109259259259259259e-2 * t22305 - 0.13265555555555555555e-1 * t22308 + 0.13265555555555555555e-1 * t22312;
    (t22312, t22314)
}
