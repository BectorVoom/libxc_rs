//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1315/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1315<F: Float>(t13547: F, t18176: F, t3: F, t1518: F, t2327: F, t116: F, t4292: F, t670: F, t2371: F, t5801: F, t117: F, t13514: F, t1459: F, t1461: F, t1916: F, t1918: F, t4158: F, t4162: F, t4165: F, t572: F, t573: F, t5795: F, t5802: F, t5805: F) -> (F, F, F, F, F, F, F) {
    let t18177 = t13547 + t18176;
    let t18178 = t3 * t18177;
    let t18190 = param_d * t18177;
    let t18204 = t2327 * t1518;
    let t18207 = t116 * t4292;
    let t18208 = t18207 * t670;
    let t18211 = t5801 * t2371;
    let t18214 = t117 * t13514;
    let t18217 = 12.0 * t1459 * t5802 + 6.0 * t1459 * t5805 + 6.0 * t1461 * t5795 + t18190 * t573 + 6.0 * t18204 * t572 + 12.0 * t18208 * t572 + 6.0 * t18211 * t572 + 3.0 * t18214 * t572 + 6.0 * t1916 * t4162 + 3.0 * t1916 * t4165 + 3.0 * t1918 * t4158;
    (t18178, t18190, t18204, t18208, t18211, t18214, t18217)
}
