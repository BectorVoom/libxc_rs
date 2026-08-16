//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2585/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2585(t13142: f64, t56878: f64, t12851: f64, t1778: f64, t3766: f64, t5219: f64, t5330: f64, t3718: f64, t44546: f64, t5353: f64, t45833: f64, t58919: f64) -> (f64, f64, f64, f64, f64) {
    let t59066 = t13142 * t56878;
    let t59144 = t1778 * t12851;
    let t59162 = t5219 * t3766 * t5330;
    let t59185 = t3718 * t44546 * t5353;
    let t59186 = 0.14291339372689912324e-3_f64 * t59185;
    let t59196 = t45833 * t58919;
    (t59066, t59144, t59162, t59186, t59196)
}
