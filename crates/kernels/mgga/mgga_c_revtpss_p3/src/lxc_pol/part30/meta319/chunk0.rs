//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1315/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1315<F: Float>(t10509: F, t123: F, t2465: F, t213: F, t2760: F, t215: F, t231: F, t268: F, t836: F, t2798: F, t2722: F, t675: F) -> (F, F, F, F, F) {
    let t10510 = t123 * t10509;
    let t10511 = t2465 * t10510;
    let t10513 = t213 * t2760;
    let t10518 = t268 * t215 * t836 * t231;
    let t10519 = t2798 * t10518;
    let t10521 = t675 * t2722;
    (t10510, t10511, t10513, t10519, t10521)
}
