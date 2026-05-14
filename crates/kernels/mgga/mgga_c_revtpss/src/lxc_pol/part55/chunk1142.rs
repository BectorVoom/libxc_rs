//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1142/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1142<F: Float>(t128986: F, t128988: F, t128990: F, t128992: F, t128994: F, t129467: F, t129470: F, t2056: F, t26399: F, t28658: F, t28750: F, t29432: F, t29444: F, t34446: F, t7359: F, t7367: F, t7374: F, t7586: F, t7988: F, t8158: F) -> (F,) {
    let t131103 = -t129467 * t2056 - t129470 * t2056 - t26399 * t8158 - t28658 * t8158 - t28750 * t7586 - t29432 * t7988 - t29444 * t7359 - t34446 * t7367 - t34446 * t7374 - t128986 - t128988 - t128990 - t128992 - t128994;
    (t131103,)
}
