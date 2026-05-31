//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1280/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1280<F: Float>(t128970: F, t128974: F, t128975: F, t128977: F, t128979: F, t128981: F, t128983: F, t128986: F, t128988: F, t128990: F, t128992: F, t128994: F, t1453: F, t28927: F, t34326: F, t8568: F) -> F {
    let t128997 = t1453 * t34326 + t28927 * t8568 + t128970 - t128974 + t128975 - t128977 - t128979 - t128981 - t128983 - F::cast_from(2.0_f64) * t128986 - F::cast_from(2.0_f64) * t128988 - F::cast_from(2.0_f64) * t128990 - F::cast_from(2.0_f64) * t128992 - F::cast_from(2.0_f64) * t128994;
    t128997
}
