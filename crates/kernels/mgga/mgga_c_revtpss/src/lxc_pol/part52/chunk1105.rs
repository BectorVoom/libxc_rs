//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1105/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1105<F: Float>(t34196: F, t4254: F, t1936: F, t28586: F, t651: F, t125478: F, t125507: F, t128317: F, t128319: F, t128321: F, t128324: F, t128326: F, t128349: F, t128354: F, t128356: F, t128357: F, t128358: F, t128360: F, t128361: F, t128362: F, t128363: F, t128478: F, t128483: F, t128485: F, t28025: F, t28287: F, t28683: F, t28734: F, t28737: F, t569: F, t6985: F, t7983: F) -> (F,) {
    let t128487 = 2.0 * t4254 * t34196;
    let t128490 = 2.0 * t651 * t28586 * t1936;
    let t128491 = -2.0 * t6985 * t28737 + 2.0 * t125478 * t28287 - 2.0 * t6985 * t28734 - t128317 - t128319 - t128321 - t128324 + t128326 + (2.0 * t28025 * t7983 + 2.0 * t28683 * t6985 + 2.0 * t128349 + 2.0 * t128354 + 2.0 * t128356 + 2.0 * t128357 + 2.0 * t128358 + 2.0 * t128360 + 2.0 * t128361 + 2.0 * t128362 + 2.0 * t128363 + t128478) * t569 - t125507 - t128483 - t128485 - t128487 - t128490;
    (t128491,)
}
