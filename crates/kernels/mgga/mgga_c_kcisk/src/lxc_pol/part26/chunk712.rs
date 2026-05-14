//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 712/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk712<F: Float>(t8234: F, t8236: F, t8238: F, t8242: F, t8245: F, t8249: F, t8253: F, t8257: F, t8261: F, t8263: F, t8265: F, t8269: F, t8272: F, t8276: F, t8280: F, t8284: F) -> (F,) {
    let t8455 = 0.9375e-1 * t8234 - 0.1875e0 * t8236 + 0.125e0 * t8238 + 0.1875e0 * t8242 - 0.125e0 * t8245 - 0.9375e-1 * t8249 - 0.20833333333333333333e-1 * t8253 + 0.625e-1 * t8257 - 0.101171875e-1 * t8261 + 0.20234375e-1 * t8263 - 0.26979166666666666666e-1 * t8265 - 0.20234375e-1 * t8269 + 0.26979166666666666666e-1 * t8272 + 0.101171875e-1 * t8276 - 0.44965277777777777777e-2 * t8280 - 0.13489583333333333333e-1 * t8284;
    (t8455,)
}
