//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1284/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1284<F: Float>(t1916: F, t32773: F, t7331: F, t8118: F, t28042: F, t572: F, t7553: F, t28986: F, t7002: F, t32776: F, t127455: F, t127459: F, t127462: F, t1918: F, t2040: F, t2115: F, t28246: F, t28975: F, t28981: F, t28990: F, t32755: F) -> F {
    let t129039 = F::cast_from(6.0_f64) * t1916 * t32773;
    let t129045 = F::cast_from(6.0_f64) * t8118 * t7331;
    let t129048 = F::cast_from(6.0_f64) * t572 * t7553 * t28042;
    let t129055 = F::cast_from(6.0_f64) * t572 * t28986 * t7002;
    let t129057 = F::cast_from(6.0_f64) * t1916 * t32776;
    let t129060 = F::cast_from(3.0_f64) * t1918 * t32755 + F::cast_from(6.0_f64) * t2040 * t28975 + F::cast_from(6.0_f64) * t2040 * t28981 + F::cast_from(3.0_f64) * t2040 * t28990 + F::cast_from(3.0_f64) * t2115 * t28246 + t127455 + t127459 + t127462 + t129039 + t129045 + t129048 + t129055 + t129057;
    t129060
}
