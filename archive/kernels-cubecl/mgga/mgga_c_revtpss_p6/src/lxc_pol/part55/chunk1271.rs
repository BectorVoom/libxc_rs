//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1271/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1271<F: Float>(t111371: F, t1936: F, t572: F, t101705: F, t7547: F, t7953: F, t1916: F, t32773: F, t7331: F, t8118: F, t28042: F, t7553: F) -> (F, F, F, F, F, F) {
    let t129029 = F::cast_from(6.0_f64) * t572 * t111371 * t1936;
    let t129032 = F::cast_from(6.0_f64) * t572 * t101705 * t1936;
    let t129034 = F::cast_from(3.0_f64) * t7547 * t7953;
    let t129039 = F::cast_from(6.0_f64) * t1916 * t32773;
    let t129045 = F::cast_from(6.0_f64) * t8118 * t7331;
    let t129048 = F::cast_from(6.0_f64) * t572 * t7553 * t28042;
    (t129029, t129032, t129034, t129039, t129045, t129048)
}
