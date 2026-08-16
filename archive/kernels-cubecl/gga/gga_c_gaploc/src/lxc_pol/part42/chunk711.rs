//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 711/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk711<F: Float>(t14271: F, t2343: F, t105: F, t13260: F, t13264: F, t13267: F, t13270: F, t13275: F, t13279: F, t13280: F, t13303: F, t13741: F, t14268: F, t2268: F) -> (F, F) {
    let t14272 = t2343 * t14271;
    let t14275 = -F::cast_from(0.47425011059460249332e-2_f64) * t13741 - t13260 + t13264 - t13267 + t13270 - F::cast_from(0.28455006635676149599e-1_f64) * t105 * t14268 + t13275 - t13279 + t13280 + t13303 + F::cast_from(0.1138200265427045984e0_f64) * t2268 * t14272;
    (t14272, t14275)
}
