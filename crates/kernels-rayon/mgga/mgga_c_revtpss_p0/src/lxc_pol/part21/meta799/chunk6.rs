//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2899/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2899(t41406: f64, t52045: f64, t52047: f64, t52049: f64, t52051: f64, t52054: f64, t52057: f64, t52060: f64, t52063: f64, t52065: f64, t52068: f64, t52116: f64) -> f64 {
    let t52615 = -0.20658999999999999999e1_f64 * t52045 + 0.68863333333333333333e0_f64 * t52047 + 0.34431666666666666666e0_f64 * t52049 + 0.57386111111111111111e0_f64 * t52051 - 0.103295e1_f64 * t52054 - 0.103295e1_f64 * t52057 - 0.17215833333333333333e1_f64 * t52060 - 0.929655e1_f64 * t52063 + 0.20839e0_f64 * t52065 - 0.104195e0_f64 * t52068 + 0.69463333333333333332e-1_f64 * t41406 + 0.6311625e0_f64 * t52116;
    t52615
}
