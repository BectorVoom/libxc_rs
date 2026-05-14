//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1074/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1074<F: Float>(t122950: F, t129431: F, t129436: F, t129437: F, t129438: F, t129440: F, t129445: F, t129447: F, t129449: F, t129452: F, t1519: F, t1911: F, t29456: F, t32825: F, t32837: F, t4257: F, t6985: F) -> (F,) {
    let t129454 = -2.0 * t122950 * t1519 - 2.0 * t129431 * t1519 + t1911 * t32837 - 2.0 * t29456 * t6985 - 2.0 * t32825 * t4257 - t129436 - t129437 + 3.0 * t129438 + 3.0 * t129440 - 2.0 * t129445 - 2.0 * t129447 - 2.0 * t129449 - 2.0 * t129452;
    (t129454,)
}
