//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2762/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2762(t17083: f64, t225: f64, t5584: f64, t852: f64, t16805: f64, t68: f64, t10076: f64, t13171: f64, t13263: f64, t13381: f64, t13388: f64, t13390: f64, t13397: f64, t13456: f64, t16758: f64, t16816: f64, t16830: f64, t17030: f64, t17046: f64, t2633: f64, t4162: f64, t4281: f64, t4282: f64, t4290: f64, t4291: f64, t4292: f64, t4295: f64, t5612: f64, t812: f64, t861: f64) -> (f64, f64, f64, f64) {
    let t58143 = t17083 * t225;
    let t58166 = t852 * t5584;
    let t58181 = t16805 * t68;
    let t58194 = -t10076 * t5612 * t812 - 2.0_f64 * t13171 * t4282 * t4291 - 2.0_f64 * t13171 * t4295 * t812 - 12.0_f64 * t13263 * t13397 * t16758 - 6.0_f64 * t13263 * t13397 * t17030 - 12.0_f64 * t13397 * t16816 * t58166 + 6.0_f64 * t17030 * t2633 * t4281 - 4.0_f64 * t4162 * t4290 * t4292 - 4.0_f64 * t13381 * t16830 - 2.0_f64 * t13388 * t16830 - 2.0_f64 * t13390 * t17046 - 4.0_f64 * t13456 * t16830 - 2.0_f64 * t58181 * t861;
    (t58143, t58166, t58181, t58194)
}
