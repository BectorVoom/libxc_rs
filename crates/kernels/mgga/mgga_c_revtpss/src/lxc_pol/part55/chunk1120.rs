//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1120/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1120<F: Float>(t101705: F, t1936: F, t572: F, t7547: F, t7953: F, t1916: F, t32773: F, t7331: F, t8118: F, t28042: F, t7553: F, t28986: F, t7002: F, t32776: F, t2055: F, t4292: F) -> (F, F, F, F, F, F, F, F) {
    let t129032 = 6.0 * t572 * t101705 * t1936;
    let t129034 = 3.0 * t7547 * t7953;
    let t129039 = 6.0 * t1916 * t32773;
    let t129045 = 6.0 * t8118 * t7331;
    let t129048 = 6.0 * t572 * t7553 * t28042;
    let t129055 = 6.0 * t572 * t28986 * t7002;
    let t129057 = 6.0 * t1916 * t32776;
    let t129065 = 6.0 * t572 * t4292 * t2055 * t1936;
    (t129032, t129034, t129039, t129045, t129048, t129055, t129057, t129065)
}
