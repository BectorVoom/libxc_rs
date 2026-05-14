//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1143/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1143<F: Float>(t122820: F, t127366: F, t127369: F, t127371: F, t127373: F, t127375: F, t127378: F, t128998: F, t128999: F, t129001: F, t129008: F, t2127: F, t28586: F, t28718: F, t28939: F, t7584: F, t8065: F, t8764: F) -> (F,) {
    let t131115 = -3.0 * t122820 * t28718 - t2127 * t28586 + 3.0 * t28939 * t8764 - t7584 * t8065 - t127366 - t127369 - t127371 - t127373 - t127375 - t127378 - 2.0 * t128998 - 2.0 * t128999 - 2.0 * t129001 - 2.0 * t129008;
    (t131115,)
}
