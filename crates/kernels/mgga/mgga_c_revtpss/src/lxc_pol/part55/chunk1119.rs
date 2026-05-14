//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1119/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1119<F: Float>(t32401: F, t7732: F, t2322: F, t34025: F, t4254: F, t651: F, t7474: F, t7741: F, t34167: F, t670: F, t1459: F, t34360: F, t7547: F, t7950: F, t111371: F, t1936: F, t572: F) -> (F, F, F, F, F, F, F, F) {
    let t128994 = t7732 * t32401;
    let t128998 = t2322 * t34025;
    let t128999 = t4254 * t34025;
    let t129001 = t651 * t7474 * t7741;
    let t129008 = t651 * t34167 * t670;
    let t129018 = 6.0 * t1459 * t34360;
    let t129026 = 6.0 * t7547 * t7950;
    let t129029 = 6.0 * t572 * t111371 * t1936;
    (t128994, t128998, t128999, t129001, t129008, t129018, t129026, t129029)
}
