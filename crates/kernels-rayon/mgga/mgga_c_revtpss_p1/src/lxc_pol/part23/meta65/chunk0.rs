//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 455/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk455(t1248: f64, t1287: f64, t487: f64, t1269: f64, t489: f64, t1204: f64, t1234: f64, t1281: f64, t1285: f64, t460: f64, t490: f64) -> (f64, f64, f64) {
    let t1288 = t487 * t1248 * t1287;
    let t1291 = t489 * t1269;
    let t1294 = 0.65854491829355115987e0_f64 * t1204 * t490 - 0.65854491829355115987e0_f64 * t1234 * t1281 + 0.65854491829355115987e0_f64 * t1285 * t1288 + 0.65854491829355115987e0_f64 * t460 * t1291;
    (t1288, t1291, t1294)
}
