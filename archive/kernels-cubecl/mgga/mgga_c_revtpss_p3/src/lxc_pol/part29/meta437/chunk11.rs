//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1637/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1637<F: Float>(t1294: F, t5245: F, t1277: F, t1774: F, t3737: F, t3738: F, t460: F, t5412: F, t17306: F, t487: F, t1269: F, t5219: F) -> (F, F, F, F, F) {
    let t18042 = t5245 * t1294;
    let t18043 = t1277 * t18042;
    let t18047 = t3737 * t1774 * t3738;
    let t18054 = t460 * t5412;
    let t18059 = t17306 * t487;
    let t18062 = t5219 * t1269;
    (t18043, t18047, t18054, t18059, t18062)
}
