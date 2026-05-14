//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 966/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk966<F: Float>(t32114: F, t7898: F, t28021: F, t8568: F, t8567: F, t8995: F, t28199: F, t125442: F, t125444: F, t125456: F, t125459: F, t125467: F, t125470: F, t125472: F, t1518: F, t2322: F, t25805: F, t27145: F, t28025: F, t28050: F, t32095: F, t33584: F, t4246: F, t4254: F, t651: F, t6985: F, t7746: F, t8557: F) -> (F,) {
    let t125474 = 2.0 * t7898 * t32114;
    let t125475 = t8568 * t28021;
    let t125478 = t8567 * t8995;
    let t125479 = t125478 * t28199;
    let t125481 = -2.0 * t1518 * t32095 * t651 - 2.0 * t2322 * t33584 - 4.0 * t25805 * t7746 - 4.0 * t27145 * t6985 - 4.0 * t28025 * t7746 - 4.0 * t28050 * t6985 - 2.0 * t33584 * t4254 - t4246 * t8557 - 4.0 * t125442 - 4.0 * t125444 - t125456 - 4.0 * t125459 - 2.0 * t125467 - t125470 + t125472 - t125474 + 2.0 * t125475 + 4.0 * t125479;
    (t125481,)
}
