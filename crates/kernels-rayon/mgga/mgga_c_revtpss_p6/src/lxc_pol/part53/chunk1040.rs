//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1040/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1040(t1937: f64, t27060: f64, t118: f64, t1310: f64, t1453: f64, t1932: f64, t2007: f64, t2127: f64, t2163: f64, t32791: f64, t32815: f64, t32823: f64, t32824: f64, t32837: f64, t508: f64, t569: f64, t649: f64, t6983: f64, t7221: f64, t7584: f64, t7683: f64, t8741: f64, t8756: f64, t8761: f64) -> f64 {
    let t32840 = t27060 * t1937;
    let t32842 = -t118 * t32791 - t1310 * t8741 + t1453 * t8761 - t1932 * t7683 - t2007 * t7584 - t2127 * t7221 - t2163 * t6983 - t32815 * t508 + t32837 * t569 - t649 * t8756 + t32823 + t32824 - 2.0_f64 * t32840;
    t32842
}
