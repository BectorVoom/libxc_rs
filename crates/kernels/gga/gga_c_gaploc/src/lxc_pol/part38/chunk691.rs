//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 691/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk691<F: Float>(t13319: F, t2268: F, t13296: F, t493: F, t492: F, t3531: F, t894: F, t12831: F, t105: F, t12821: F, t13303: F, t13306: F, t13309: F, t13312: F, t13315: F, t13316: F) -> (F, F, F, F) {
    let t13321 = F::new(0.28455006635676149599e-1) * t2268 * t13319;
    let t13322 = t493 * t13296;
    let t13323 = t492 * t13322;
    let t13327 = t894 * t3531;
    let t13329 = F::new(0.28455006635676149599e-1) * t2268 * t13327;
    let t13330 = F::new(0.142275033178380748e-1) * t12831;
    let t13331 = t13303 + t13306 - t13309 + t13312 - t13315 + F::new(0.56910013271352299198e-1) * t2268 * t13316 + t13321 - F::new(0.28455006635676149599e-1) * t105 * t13323 - F::new(0.47425011059460249332e-2) * t12821 + t13329 - t13330;
    (t13322, t13323, t13327, t13331)
}
