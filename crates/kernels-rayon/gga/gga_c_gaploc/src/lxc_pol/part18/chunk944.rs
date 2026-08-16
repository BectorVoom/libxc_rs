//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 944/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk944(t10148: f64, t10181: f64, t10230: f64, t10279: f64, t209: f64, t3362: f64, t501: f64, t605: f64, t8042: f64, t921: f64, t2358: f64, t8045: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10281 = t10148 + t10181 + t10230 + t10279;
    let t10282 = t10281 * t209;
    let t10283 = t3362 * t501;
    let t10284 = t10283 * t605;
    let t10285 = t8042 * t921;
    let t10286 = t8045 * t2358;
    (t10281, t10282, t10283, t10284, t10285, t10286)
}
