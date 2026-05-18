//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1363/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1363<F: Float>(t1042: F, t17221: F, t3172: F, t5269: F, t1261: F, t13396: F, t5268: F, t12256: F, t13099: F, t15936: F, t1224: F, t140: F) -> (F, F, F, F, F) {
    let t17222 = t1042 * t17221;
    let t17225 = t3172 * t5269;
    let t17227 = F::new(0.3811023832717309953e-3) * t1261 * t17225;
    let t17231 = t5268 * t13396;
    let t17232 = t1042 * t17231;
    let t17235 = t13099 * t12256;
    let t17236 = t17235 * t15936;
    let t17237 = t1042 * t17236;
    let t17240 = t140 * t1224;
    (t17222, t17227, t17232, t17237, t17240)
}
