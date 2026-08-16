//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1145/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1145(t121336: f64, t121305: f64, t32186: f64, t7063: f64, t119900: f64, t121165: f64, t240: f64, t545: f64, t1412: f64, t844: f64, t32291: f64, t8591: f64) -> (f64, f64, f64, f64, f64) {
    let t121337 = 0.1054086758983270768e-1_f64 * t121336;
    let t121342 = t7063 * t121305 * t32186;
    let t121346 = t119900 * t545 * t240 * t121165;
    let t121354 = t844 * t1412;
    let t121356 = t8591 * t121354 * t32291;
    (t121337, t121342, t121346, t121354, t121356)
}
