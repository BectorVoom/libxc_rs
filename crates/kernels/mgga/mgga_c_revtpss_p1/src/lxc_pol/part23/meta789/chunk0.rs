//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2603/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2603<F: Float>(t11044: F, t18797: F, t18317: F, t2435: F, t10871: F, t5977: F, t14931: F, t18477: F, t51123: F, t10811: F, t18471: F, t18451: F) -> (F, F, F, F, F, F) {
    let t61441 = t11044 * t18797;
    let t61448 = t2435 * t18317;
    let t61532 = t5977 * t10871;
    let t61538 = t14931 * t51123 * t18477;
    let t61540 = t10811 * t18471;
    let t61542 = t10811 * t18451;
    (t61441, t61448, t61532, t61538, t61540, t61542)
}
