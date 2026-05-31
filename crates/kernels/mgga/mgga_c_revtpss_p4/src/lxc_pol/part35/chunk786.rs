//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 786/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk786<F: Float>(t13180: F, t225: F, t1466: F, t2246: F, t1514: F, t2289: F, t1857: F, t3857: F, t2516: F, t5571: F, t1320: F, t5569: F) -> (F, F, F, F, F, F) {
    let t13181 = F::cast_from(1.0_f64) / t13180;
    let t13182 = t225 * t13181;
    let t13272 = t1466 * t2246;
    let t13448 = t2289 * t1514;
    let t13584 = t3857 * t1857;
    let t13611 = t5571 * t2516;
    let t13621 = t1320 * t5569;
    (t13182, t13272, t13448, t13584, t13611, t13621)
}
