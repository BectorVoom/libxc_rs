//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 869/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk869<F: Float>(t2089: F, t4292: F, t670: F, t8065: F, t1518: F, t7474: F, t1519: F, t2322: F, t26399: F, t28658: F, t4254: F, t4257: F, t651: F, t7235: F, t7359: F, t7374: F, t7537: F, t7539: F, t7732: F, t7898: F, t7978: F, t7988: F, t8111: F) -> (F, F, F, F) {
    let t28734 = t2089 * t4292;
    let t28737 = t8065 * t670;
    let t28750 = t7474 * t1518;
    let t28759 = -2.0 * t1519 * t26399 - 2.0 * t1519 * t28658 - 2.0 * t2322 * t7978 - 2.0 * t2322 * t7988 - 2.0 * t28734 * t651 - 2.0 * t28737 * t651 - 2.0 * t28750 * t651 - 2.0 * t4254 * t7978 - 2.0 * t4254 * t7988 - 2.0 * t4257 * t7359 - t7235 * t8111 - 2.0 * t7374 * t7732 + t7537 * t7898 - t7539 * t7898;
    (t28734, t28737, t28750, t28759)
}
