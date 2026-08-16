//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1142/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1142<F: Float>(t5940: F, t705: F, t707: F, t10605: F, t6002: F, t2411: F, t6079: F, t10446: F, t5819: F, t2375: F, t5825: F, t13309: F, t13310: F) -> (F, F, F, F, F, F) {
    let t18263 = t705 * t5940;
    let t18265 = F::cast_from(4.0_f64) * t18263 * t707;
    let t18267 = F::cast_from(12.0_f64) * t10605 * t6002;
    let t18268 = t6079 * t2411;
    let t18272 = t10446 * t5819;
    let t18277 = t2375 * t5825;
    let t18280 = -t13309 - t13310;
    (t18265, t18267, t18268, t18272, t18277, t18280)
}
