//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 366/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk366<F: Float>(t1678: F, t225: F, t385: F, t1082: F, t1651: F, t1089: F, t1668: F, t378: F, t380: F, t1024: F, t1087: F, t1647: F, t342: F, t381: F) -> (F, F, F, F, F) {
    let t1679 = t1678 * t225;
    let t1680 = t1679 * t385;
    let t1685 = t1082 * t1651;
    let t1689 = t378 * t1668 * t1089;
    let t1692 = t380 * t1678;
    let t1695 = F::cast_from(0.65854491829355115987e0_f64) * t1647 * t381 - F::cast_from(0.65854491829355115987e0_f64) * t1024 * t1685 + F::cast_from(0.65854491829355115987e0_f64) * t1087 * t1689 + F::cast_from(0.65854491829355115987e0_f64) * t342 * t1692;
    (t1680, t1685, t1689, t1692, t1695)
}
