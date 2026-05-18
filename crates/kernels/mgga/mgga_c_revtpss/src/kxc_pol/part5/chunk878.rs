//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 878/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk878<F: Float>(t373: F, t6305: F, t3155: F, t1042: F, t3162: F, t225: F, t6235: F, t366: F, t1066: F, t6100: F, t247: F, t3182: F, t6092: F) -> (F, F, F, F, F, F, F, F) {
    let t6306 = t373 * t6305;
    let t6307 = t6306 * t3155;
    let t6308 = t1042 * t6307;
    let t6311 = t6306 * t3162;
    let t6312 = t1042 * t6311;
    let t6317 = t6235 * t225;
    let t6318 = t6317 * t366;
    let t6322 = t1066 * t6100;
    let t6323 = t247 * t6322;
    let t6326 = t3182 * t6092;
    (t6307, t6308, t6311, t6312, t6317, t6318, t6323, t6326)
}
