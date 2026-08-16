//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1243/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1243(t18402: f64, t25234: f64, t18409: f64, t25227: f64, t2661: f64, t25266: f64, t5980: f64, t18531: f64, t25245: f64, t18432: f64, t93025: f64, t18440: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t106037 = t25234 * t18402;
    let t106040 = t2661 * t25227 * t18409;
    let t106042 = t25266 * t5980;
    let t106048 = t25245 * t18531;
    let t106050 = t93025 * t18432;
    let t106053 = t2661 * t25227 * t18440;
    (t106037, t106040, t106042, t106048, t106050, t106053)
}
