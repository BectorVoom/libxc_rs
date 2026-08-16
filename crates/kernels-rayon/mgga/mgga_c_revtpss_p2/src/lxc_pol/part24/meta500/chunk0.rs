//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1503/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1503(t1558: f64, t231: f64, t6016: f64, t2782: f64, t2797: f64, t23167: f64, t251: f64, t2783: f64, t76131: f64, t18719: f64, t51549: f64, t23245: f64, t2798: f64, t686: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t76161 = t6016 * t1558 * t231;
    let t76163 = t2782 * t2797 * t76161;
    let t76169 = t251 * t23167;
    let t76172 = t2782 * t2783 * t76169 * t231;
    let t76182 = t2782 * t2783 * t76131 * t231;
    let t76206 = t51549 * t18719;
    let t76223 = t2798 * t23245 * t72 * t686;
    (t76163, t76169, t76172, t76182, t76206, t76223)
}
