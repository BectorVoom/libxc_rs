//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 621/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk621<F: Float>(t3531: F, t894: F, t2268: F, t12831: F, t105: F, t12821: F, t13303: F, t13306: F, t13309: F, t13312: F, t13315: F, t13316: F, t13321: F, t13323: F, t13302: F, t209: F) -> (F, F, F) {
    let t13327 = t894 * t3531;
    let t13329 = 0.28455006635676149599e-1 * t2268 * t13327;
    let t13330 = 0.142275033178380748e-1 * t12831;
    let t13331 = t13303 + t13306 - t13309 + t13312 - t13315 + 0.56910013271352299198e-1 * t2268 * t13316 + t13321 - 0.28455006635676149599e-1 * t105 * t13323 - 0.47425011059460249332e-2 * t12821 + t13329 - t13330;
    let t13332 = t13302 + t13331;
    let t13333 = t13332 * t209;
    (t13327, t13332, t13333)
}
