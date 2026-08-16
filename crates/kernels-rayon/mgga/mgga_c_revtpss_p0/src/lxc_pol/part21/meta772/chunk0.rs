//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2742/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2742(t14869: f64, t9775: f64, t10899: f64, t136: f64, t216: f64, t14786: f64, t231: f64, t40834: f64, t854: f64, t14833: f64, t236: f64, t2453: f64, t9794: f64) -> (f64, f64, f64, f64, f64) {
    let t50443 = t9775 * t14869;
    let t50446 = t216 * t10899 * t136;
    let t50451 = t14786 * t231;
    let t50453 = t40834 * t854 * t50451;
    let t50454 = 0.30492001685571196935e-4_f64 * t50453;
    let t50457 = t2453 * t236 * t9794 * t14833;
    (t50443, t50446, t50451, t50454, t50457)
}
