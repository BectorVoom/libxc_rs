//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 993/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk993<F: Float>(t2055: F, t8233: F, t2163: F, t7983: F, t1519: F, t2052: F, t2056: F, t29427: F, t33287: F, t34329: F, t34446: F, t4248: F, t651: F, t7586: F, t7732: F, t7978: F, t7984: F, t7988: F, t8079: F, t8111: F, t8764: F, t8892: F) -> (F, F, F) {
    let t34821 = t8233 * t2055;
    let t34824 = t2163 * t7983;
    let t34827 = -2.0 * t1519 * t33287 - t2052 * t8233 - 2.0 * t2056 * t29427 - 2.0 * t2056 * t34446 - 2.0 * t34821 * t651 - 2.0 * t34824 * t651 - 2.0 * t4248 * t8892 - 2.0 * t7586 * t7978 - 2.0 * t7586 * t7984 - 2.0 * t7586 * t7988 - 2.0 * t7732 * t8892 + 3.0 * t8079 * t8764 - t8111 * t8764 - t34329;
    (t34821, t34824, t34827)
}
