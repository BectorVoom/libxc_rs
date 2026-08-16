//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1111/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1111(t2055: f64, t8233: f64, t2163: f64, t7983: f64, t1519: f64, t2052: f64, t2056: f64, t29427: f64, t33287: f64, t34329: f64, t34446: f64, t4248: f64, t651: f64, t7586: f64, t7732: f64, t7978: f64, t7984: f64, t7988: f64, t8079: f64, t8111: f64, t8764: f64, t8892: f64) -> (f64, f64, f64) {
    let t34821 = t8233 * t2055;
    let t34824 = t2163 * t7983;
    let t34827 = -2.0_f64 * t1519 * t33287 - t2052 * t8233 - 2.0_f64 * t2056 * t29427 - 2.0_f64 * t2056 * t34446 - 2.0_f64 * t34821 * t651 - 2.0_f64 * t34824 * t651 - 2.0_f64 * t4248 * t8892 - 2.0_f64 * t7586 * t7978 - 2.0_f64 * t7586 * t7984 - 2.0_f64 * t7586 * t7988 - 2.0_f64 * t7732 * t8892 + 3.0_f64 * t8079 * t8764 - t8111 * t8764 - t34329;
    (t34821, t34824, t34827)
}
