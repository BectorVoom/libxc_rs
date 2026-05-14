//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 679/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk679<F: Float>(t1783: F, t7268: F, t1310: F, t1773: F, t2460: F, t2466: F, t4989: F, t5003: F, t5013: F, t7231: F, t7235: F, t7239: F, t7243: F, t7248: F, t7254: F, t7258: F, t7264: F) -> (F, F, F) {
    let t7269 = t1783 * t7268;
    let t7270 = t1310 * t7269;
    let t7273 = -0.17990788716177317213e-1 * t5003 + 0.17990788716177317213e-1 * t4989 * t2460 + 0.59969295720591057377e-2 * t7231 + 0.23987718288236422951e-1 * t5013 * t7235 - 0.17990788716177317213e-1 * t5013 * t7239 - 0.35981577432354634426e-1 * t5013 * t7243 - 0.35981577432354634426e-1 * t1773 * t7248 - 0.5397236614853195164e-1 * t4989 * t2466 - 0.17990788716177317213e-1 * t7254 - 0.17990788716177317213e-1 * t5013 * t7258 + 0.10794473229706390328e0 * t1773 * t7264 - 0.5397236614853195164e-1 * t1773 * t7270;
    (t7269, t7270, t7273)
}
