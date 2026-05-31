//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 503/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk503<F: Float>(t225: F, t3555: F, t480: F, t3566: F, t1236: F, t127: F, t371: F, t1235: F, t221: F, t462: F, t696: F, t461: F) -> (F, F, F, F, F, F, F) {
    let t3666 = t3555 * t225;
    let t3667 = t3666 * t480;
    let t3670 = t3566 * t225;
    let t3678 = t371 * t127 * t1236;
    let t3679 = t1235 * t3678;
    let t3682 = t221 * t696 * t462;
    let t3684 = t461 * t3682 / F::cast_from(432.0_f64);
    (t3666, t3667, t3670, t3678, t3679, t3682, t3684)
}
