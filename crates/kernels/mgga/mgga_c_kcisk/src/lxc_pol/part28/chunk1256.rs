//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1256/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1256<F: Float>(t2815: F, t9262: F, t12352: F, t33153: F, t35263: F, t35265: F, t35267: F, t35268: F, t35271: F, t35276: F, t35279: F, t35282: F, t35338: F, t240: F, t35269: F, t35273: F, t35523: F) -> (F, F) {
    let t35526 = t2815 * t9262;
    let t35529 = -6.0 * t12352 * t35526 + 2.0 * t33153 * t9262 - t35263 + t35265 - t35267 + t35268 - t35271 + t35276 - t35279 - t35282 + t35338;
    let t35532 = t35263 - t35265 + t35267 - t35268 - t35269 + t35271 - t35273 - t35276 + t35279 + t35282 - t35338 + t240 * (t35523 + t35529);
    (t35526, t35532)
}
