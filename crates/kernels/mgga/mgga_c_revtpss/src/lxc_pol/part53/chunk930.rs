//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 930/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk930<F: Float>(t3: F, t32885: F, t2042: F, t7696: F, t2170: F, t7331: F, t7334: F, t1461: F, t32358: F, t32360: F, t32362: F, t32365: F, t32368: F, t32371: F, t32373: F, t32377: F, t573: F, t8616: F, t8771: F) -> (F, F, F) {
    let t32886 = t3 * t32885;
    let t32897 = param_d * t32885;
    let t32901 = t7696 * t2042;
    let t32903 = t2170 * t7331;
    let t32905 = t2170 * t7334;
    let t32910 = 3.0 * t1461 * t8771 + t32897 * t573 + 3.0 * t32358 + 6.0 * t32360 + 3.0 * t32362 + t32365 + t32368 + t32371 + t32373 + t32377 + 3.0 * t32901 + 6.0 * t32903 + 3.0 * t32905 + t8616;
    (t32886, t32897, t32910)
}
