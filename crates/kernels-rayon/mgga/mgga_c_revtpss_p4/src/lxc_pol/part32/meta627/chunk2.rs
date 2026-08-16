//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2004/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2004(t108688: f64, t1310: f64, t1518: f64, t18235: f64, t18245: f64, t2056: f64, t2322: f64, t27123: f64, t27126: f64, t28196: f64, t28286: f64, t28586: f64, t28696: f64, t28760: f64, t29508: f64, t30570: f64, t30571: f64, t30578: f64, t4248: f64, t4254: f64, t4292: f64, t651: f64, t6765: f64, t7359: f64, t7367: f64, t7373: f64, t7374: f64, t7378: f64, t75439: f64, t7732: f64, t7978: f64, t8065: f64, t85360: f64) -> f64 {
    let t110102 = -4.0_f64 * t7359 * t18235 - 2.0_f64 * t18245 * t7378 - 2.0_f64 * t75439 * t2056 - 2.0_f64 * t85360 * t2056 - 2.0_f64 * t18245 * t7367 - 4.0_f64 * t651 * t8065 * t4292 - 4.0_f64 * t2322 * t30578 - 4.0_f64 * t4254 * t30578 - 4.0_f64 * t651 * t28586 * t1518 - 2.0_f64 * t651 * t6765 * t7373 - 2.0_f64 * t2322 * t30571 - 2.0_f64 * t4254 * t30571 - 2.0_f64 * t651 * t1310 * t30570 - 2.0_f64 * t29508 * t7374 - 4.0_f64 * t27123 * t7978 - 4.0_f64 * t27126 * t7978 - 4.0_f64 * t7732 * t28760 + 4.0_f64 * t28196 * t28286 * t108688 - 4.0_f64 * t4248 * t28696;
    t110102
}
