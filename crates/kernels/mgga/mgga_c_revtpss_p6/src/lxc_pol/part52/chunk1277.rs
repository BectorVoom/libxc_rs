//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1277/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1277<F: Float>(t27833: F, t8715: F, t32735: F, t7898: F, t125362: F, t125365: F, t128945: F, t128959: F, t128960: F, t128964: F, t1932: F, t2056: F, t28030: F, t28586: F, t32322: F, t33602: F, t651: F, t6983: F, t7367: F, t7373: F, t7374: F, t7883: F, t8065: F, t8109: F) -> F {
    let t128965 = t27833 * t8715;
    let t128966 = t7898 * t32735;
    let t128967 = -F::new(2.0) * t651 * t7373 * t7883 - F::new(2.0) * t125362 * t2056 - F::new(2.0) * t125365 * t2056 - t1932 * t28586 - F::new(2.0) * t28030 * t7374 + t32322 * t8109 - F::new(2.0) * t33602 * t7367 - t6983 * t8065 + t128945 + t128959 + t128960 + t128964 + t128965 + t128966;
    t128967
}
