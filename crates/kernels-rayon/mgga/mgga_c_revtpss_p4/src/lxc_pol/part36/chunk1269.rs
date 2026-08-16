//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1269/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1269(t25266: f64, t5980: f64, t18531: f64, t25245: f64, t18432: f64, t93025: f64, t18440: f64, t25227: f64, t2661: f64, t18348: f64, t1945: f64, t807: f64) -> (f64, f64, f64, f64, f64) {
    let t106042 = t25266 * t5980;
    let t106048 = t25245 * t18531;
    let t106050 = t93025 * t18432;
    let t106053 = t2661 * t25227 * t18440;
    let t106061 = t807 * t1945 * t18348;
    (t106042, t106048, t106050, t106053, t106061)
}
