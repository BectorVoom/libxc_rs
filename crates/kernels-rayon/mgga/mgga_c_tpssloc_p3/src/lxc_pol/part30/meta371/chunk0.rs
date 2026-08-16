//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1420/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1420(t1788: f64, t2225: f64, t2221: f64, t225: f64, t5213: f64, t5211: f64, t1372: f64, t1824: f64, t5286: f64, t562: f64, t12248: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15982 = t2225 * t1788;
    let t15984 = t2221 * t1788;
    let t16022 = t5213 * t225;
    let t16030 = t5211 * t225;
    let t16036 = t1372 * t1824;
    let t16040 = t562 * t5286;
    let t16046 = t68 * t12248;
    (t15982, t15984, t16022, t16030, t16036, t16040, t16046)
}
