//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 867/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk867<F: Float>(t1843: F, t7373: F, t118: F, t13426: F, t1502: F, t18227: F, t1911: F, t2052: F, t2056: F, t2089: F, t25082: F, t28196: F, t28287: F, t28586: F, t28588: F, t28653: F, t28686: F, t4246: F, t4248: F, t5517: F, t569: F, t651: F, t671: F, t7357: F, t7367: F, t7474: F, t7484: F) -> (F, F) {
    let t28696 = t1843 * t7373;
    let t28699 = -t118 * t28586 - 2.0 * t13426 * t2056 - t1502 * t7474 - 2.0 * t18227 * t2056 - t1843 * t7357 + t1911 * t7484 - t2052 * t5517 - t2089 * t4246 - 3.0 * t25082 * t28588 + 2.0 * t28196 * t28287 - 2.0 * t28653 * t671 + t28686 * t569 - 2.0 * t28696 * t651 - 2.0 * t4248 * t7367;
    (t28696, t28699)
}
