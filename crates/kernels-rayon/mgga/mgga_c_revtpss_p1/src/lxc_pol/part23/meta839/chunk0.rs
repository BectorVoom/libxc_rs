//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2712/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2712(t1263: f64, t372: f64, t6628: f64, t21233: f64, t3647: f64, t17451: f64, t17605: f64, t17209: f64, t17569: f64, t20824: f64, t3172: f64, t3711: f64) -> (f64, f64, f64, f64, f64) {
    let t69839 = t372 * t1263 * t6628;
    let t69856 = t3647 * t21233;
    let t69866 = t17605 * t17451;
    let t69885 = t17569 * t17209;
    let t69890 = t3711 * t3172 * t20824;
    (t69839, t69856, t69866, t69885, t69890)
}
