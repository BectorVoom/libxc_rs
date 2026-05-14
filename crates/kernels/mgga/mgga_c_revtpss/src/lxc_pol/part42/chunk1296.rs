//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1296/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1296<F: Float>(t30: F, t33: F, t1344: F, t18280: F, t21944: F, t21949: F, t2255: F, t5574: F, t605: F, t6792: F, t9617: F, t3881: F, t6416: F, t1113: F, t1348: F, t20256: F, t5582: F, t1882: F, t1892: F, zeta_threshold: F) -> (F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t21955 = piecewise3(t31, 0.0, 8.0 / 27.0 * t21944 * t605 - 8.0 / 9.0 * t5574 * t2255 - 2.0 / 9.0 * t21949 * t605 + 2.0 / 3.0 * t1344 * t18280);
    let t21956 = t9617 * t6792;
    let t21961 = t3881 * t6416;
    let t21967 = piecewise3(t34, 0.0, 8.0 / 27.0 * t21956 * t1113 + 8.0 / 9.0 * t5582 * t2255 - 2.0 / 9.0 * t21961 * t1113 + 2.0 / 3.0 * t1348 * t20256);
    let t21969 = t21955 / 2.0 + t21967 / 2.0;
    let t21981 = t1892 * t1882;
    (t21969, t21981)
}
