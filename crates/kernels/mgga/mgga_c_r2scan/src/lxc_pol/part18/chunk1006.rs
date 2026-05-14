//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1006/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1006<F: Float>(t12398: F, t12406: F, t12410: F, t12413: F, t12417: F, t12420: F, t12585: F, t39157: F, t39159: F, t39160: F, t39161: F, t39162: F, t39163: F, t39164: F, t39167: F, t39168: F) -> (F,) {
    let t42377 = t39157 + t39159 - t12398 + t39160 + t39161 - t39162 - t12406 + t12585 + t12410 + t39163 + t39164 - t39167 - t39168 - t12413 + t12417 + t12420;
    (t42377,)
}
