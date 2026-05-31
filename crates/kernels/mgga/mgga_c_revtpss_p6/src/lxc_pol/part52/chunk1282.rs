//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1282/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1282<F: Float>(t127540: F, t128201: F, t128232: F, t128262: F, t128288: F, t128307: F, t128491: F, t128522: F, t128555: F, t128872: F, t128897: F, t128911: F, t128941: F, t128967: F, t128997: F, t129009: F) -> F {
    let t129014 = t127540 + t128201 + t128232 + t128262 + t128288 + F::cast_from(2.0_f64) * t128307 + t128491 + t128522 + t128555 + t128872 + t128897 + t128911 + t128941 + t128967 + t128997 + F::cast_from(2.0_f64) * t129009;
    t129014
}
