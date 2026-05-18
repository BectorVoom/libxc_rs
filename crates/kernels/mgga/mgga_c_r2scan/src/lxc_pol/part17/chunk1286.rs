//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1286/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1286<F: Float>(t1115: F, t3060: F, t36967: F, t3269: F, t12739: F, t42916: F, t10610: F, t11199: F, t12414: F, t12056: F, t3275: F, t7040: F) -> (F, F, F, F) {
    let t45081 = t36967 * t1115 * t3060;
    let t45083 = F::new(45.0) / F::new(64.0) * t3269 * t45081;
    let t45085 = F::new(3.0) / F::new(2.0) * t42916 * t12739;
    let t45088 = F::new(3.0) / F::new(2.0) * t10610 * t11199 * t12414;
    let t45094 = t3275 * t12056 * t7040 / F::new(2.0);
    (t45083, t45085, t45088, t45094)
}
