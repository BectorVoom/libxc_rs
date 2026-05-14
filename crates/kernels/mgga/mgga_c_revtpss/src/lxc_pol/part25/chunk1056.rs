//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1056/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1056<F: Float>(t13435: F, t1936: F, t2322: F, t7002: F, t13440: F, t5523: F, t112: F, t239: F, t624: F, t655: F, t665: F, t2339: F, t68: F, t2340: F, t2366: F, t6998: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t25814 = 4.0 * t13435 * t1936;
    let t25816 = 4.0 * t2322 * t7002;
    let t25818 = 2.0 * t13440 * t1936;
    let t25820 = 4.0 * t5523 * t7002;
    let t25821 = t239 * t112;
    let t25822 = 11.0 / 9.0 * t25821;
    let t25823 = t624 * t655;
    let t25824 = t25823 * t665;
    let t25825 = 2.0 / 3.0 * t25824;
    let t25826 = t68 * t2339;
    let t25827 = t25826 * t2340;
    let t25829 = t6998 * t2366;
    (t25814, t25816, t25818, t25820, t25822, t25823, t25825, t25826, t25827, t25829)
}
