//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 619/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk619<F: Float>(t13296: F, t169: F, t172: F, t452: F, t105: F, t12771: F, t13250: F, t13254: F, t13260: F, t13264: F, t13267: F, t13270: F, t13275: F, t13279: F, t13280: F, t2268: F) -> (F, F, F) {
    let t13298 = t13296 * t169 * t172;
    let t13299 = t452 * t13298;
    let t13302 = -0.17073003981405689759e0 * t2268 * t13250 + 0.1138200265427045984e0 * t2268 * t13254 - t13260 + t13264 - t13267 + t13270 + 0.47425011059460249332e-2 * t12771 + t13275 - t13279 + t13280 + 0.28455006635676149599e-1 * t105 * t13299;
    (t13298, t13299, t13302)
}
