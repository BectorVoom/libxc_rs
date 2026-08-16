//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1497/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1497(t18322: f64, t18791: f64, t18810: f64, t18836: f64, t10563: f64, t10566: f64, t14324: f64, t14343: f64, t14345: f64, t14372: f64, t18392: f64, t18535: f64, t18536: f64, t18537: f64, t18538: f64, t18541: f64, t18543: f64, t18546: f64, t18548: f64, t18549: f64, t18552: f64, t198: f64, t207: f64, t2403: f64, t4343: f64, t4546: f64, t765: f64, t892: f64, t9394: f64) -> (f64, f64) {
    let t18838 = t18322 + t18791 + t18810 + t18836;
    let t18848 = t18838 * t198 * t207 * t892 + 3.0_f64 * t18392 * t198 * t765 + 6.0_f64 * t2403 * t4343 * t4546 + t10563 + t10566 - t14324 + t14343 + t14345 + t14372 + t18535 - t18536 - t18537 + t18538 + t18541 + t18543 + t18546 + t18548 + t18549 + t18552 + t9394;
    (t18838, t18848)
}
